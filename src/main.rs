use zkp_stark::{*, primefield::*};
use reqwest::Error;


struct FibonacciClaim {
    index: usize,
    value: FieldElement,
}

impl Verifiable for FibonacciClaim {
    fn constraints(&self) -> Constraints {
        use RationalExpression::*;

        // Seed
        let mut seed = self.index.to_be_bytes().to_vec();
        seed.extend_from_slice(&self.value.as_montgomery().to_bytes_be());

        // Constraint repetitions
        let trace_length = self.index.next_power_of_two();
        let g = Constant(FieldElement::root(trace_length).unwrap());
        let on_row = |index| (X - g.pow(index)).inv();
        let every_row = || (X - g.pow(trace_length - 1)) / (X.pow(trace_length) - 1.into());

        let c = Constraints::from_expressions((trace_length, 2), seed, vec![
            (Trace(0, 1) - Trace(1, 0)) * every_row(),
            (Trace(1, 1) - Trace(0, 0) - Trace(1, 0)) * every_row(),
            (Trace(0, 0) - 1.into()) * on_row(0),
            (Trace(0, 0) - (&self.value).into()) * on_row(self.index),
        ])
        .unwrap();
        return c
    }
}

impl Provable<&FieldElement> for FibonacciClaim {
    fn trace(&self, witness: &FieldElement) -> TraceTable {
        let trace_length = self.index.next_power_of_two();
        let mut trace = TraceTable::new(trace_length, 2);
        trace[(0, 0)] = 1.into();
        trace[(0, 1)] = witness.clone();
        for i in 0..(trace_length - 1) {
            trace[(i + 1, 0)] = trace[(i, 1)].clone();
            trace[(i + 1, 1)] = &trace[(i, 0)] + &trace[(i, 1)];
        }
        trace
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let claim = FibonacciClaim {
        index: 5000,
        value: FieldElement::from_hex_str("069673d708ad3174714a2c27ffdb56f9b3bfb38c1ea062e070c3ace63e9e26eb"),
    };

    let secret = FieldElement::from(40);

    let proof = claim.prove(&secret).unwrap();
    
    let proof_bytes = proof.as_bytes(); // Get the byte slice
    let proof_hex = hex::encode(proof_bytes); // Convert byte slice to hex string
    
    let serialized_proof = serde_json::json!({
            "proof": proof_hex
    });
    
    let client = reqwest::Client::new();
    println!("Hello, world! 45666");
    let res = client.post("http://localhost:8080/submit_proof")
        .json(&serialized_proof)
        .send()
        .await?;

        if res.status().is_success() {
            let body = res.text().await?; // For text response
            println!("Response body: {}", body);
        } else {
            println!("Request failed with status: {}", res.status());
        }
        
        Ok(())
    
}
