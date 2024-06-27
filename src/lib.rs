use pgrx::*;
use serde_json::Value;
use zen_engine::{DecisionEngine, model::DecisionContent};
use futures::executor::block_on;

pg_module_magic!();

#[pg_extern]
fn evaluate_jdm(graph: JsonB, data: JsonB) -> JsonB {
    let decision_content: DecisionContent = serde_json::from_value(graph.0).unwrap();
    let engine = DecisionEngine::default();
    let decision = engine.create_decision(decision_content.into());

    let result = block_on(decision.evaluate(&data.0)).unwrap();

    // Manually convert the result to serde_json::Value
    let result_value: Value = serde_json::to_value(result).unwrap();
    JsonB(result_value)
}

#[pg_schema]
#[cfg(any(test, feature = "pg_test"))]
mod tests {
    use super::*;
    use pgrx::*;
    use serde_json::json;

    #[pg_test]
    fn test_evaluate_jdm() {
        let graph = json!({
            "contentType": "application/vnd.gorules.decision",
            "edges": [
              {
                "id": "1",
                "type": "edge",
                "sourceId": "inputNode1",
                "targetId": "decisionNode1"
              },
              {
                "id": "2",
                "type": "edge",
                "sourceId": "decisionNode1",
                "targetId": "outputNode1"
              }
            ],
            "nodes": [
              {
                "id": "inputNode1",
                "name": "Input Value",
                "type": "inputNode",
                "position": {
                  "x": 100,
                  "y": 100
                }
              },
              {
                "id": "decisionNode1",
                "name": "Check Value",
                "type": "decisionTableNode",
                "content": {
                  "rules": [
                    {
                      "_id": "rule1",
                      "valueCheck": "< 10",
                      "result": "\"less than 10\""
                    },
                    {
                      "_id": "rule2",
                      "valueCheck": "> 10",
                      "result": "\"greater than 10\""
                    },
                    {
                      "_id": "rule3",
                      "valueCheck": "== 10",
                      "result": "\"equal to 10\""
                    }
                  ],
                  "inputs": [
                    {
                      "id": "valueCheck",
                      "name": "Value Check",
                      "type": "expression",
                      "field": "input.value"
                    }
                  ],
                  "outputs": [
                    {
                      "id": "result",
                      "name": "Result",
                      "type": "expression",
                      "field": "output.result"
                    }
                  ],
                  "hitPolicy": "first"
                },
                "position": {
                  "x": 300,
                  "y": 100
                }
              },
              {
                "id": "outputNode1",
                "name": "Output Result",
                "type": "outputNode",
                "position": {
                  "x": 500,
                  "y": 100
                }
              }
            ]
          });
        
        let data = json!({
            "input": {
                "value": 99
            }
        });
        let expected_output = json!({
            "output": {
                "result": "greater than 10"
            }
        });
        
        let evaluation = evaluate_jdm(JsonB(graph.clone()), JsonB(data.clone()));
        
        // Extract the "result" field from the evaluation object
        let evaluation_result = match evaluation.0.get("result") {
            Some(value) => value,
            None => {
                eprintln!("Key 'result' not found");
                return;
            }
        };
        
        // Extract the "output" field from the evaluation_result
        let evaluation_output = match evaluation_result.get("output") {
            Some(value) => value,
            None => {
                eprintln!("Key 'output' not found");
                return;
            }
        };
        
        assert_eq!(evaluation_output, expected_output.get("output").unwrap());


    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
        //    pgrx::spi::Spi::run("CREATE SCHEMA IF NOT EXISTS tests").expect("Failed to create schema");
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
