use crate::proto::LabelPair;
use crate::timer;
use std::collections::HashMap;

/// An OpenMetrics Exemplar
///
/// https://github.com/OpenObservability/OpenMetrics/blob/master/specification/OpenMetrics.md#exemplars
#[derive(Debug, Clone)]
pub struct Exemplar {
    pub(crate) value: f64,
    pub(crate) labels: Vec<LabelPair>,
    pub(crate) timestamp_ms: i64,
}

impl Exemplar {
    /// Create an ['Exemplar'] with value
    pub fn new(val: f64) -> Self {
        Self {
            value: val,
            labels: vec![],
            timestamp_ms: (timer::now_millis() / 1000) as i64,
        }
    }

    /// Create an ['Exemplar'] with value and labels
    pub fn new_with_labels(val: f64, exemplar_labels: HashMap<String, String>) -> Self {
        let mut label_pairs = Vec::with_capacity(exemplar_labels.len());
        for (n, v) in exemplar_labels.iter() {
            let mut label_pair = LabelPair::default();
            label_pair.set_name(n.to_string());
            label_pair.set_value(v.to_string());
            label_pairs.push(label_pair);
        }

        let ex = Self {
            value: val,
            labels: label_pairs,
            timestamp_ms: (timer::now_millis() / 1000) as i64,
        };
        // TODO: verify length of labelset + values as <= 128 UTF8 chars
        ex
    }

    //fn set(&mut self, val: P::T, labels: Vec<LabelPair>) {
    //    self.value.set(val);
    //    self.labels = labels;
    //    self.timestamp_ms = (timer::now_millis() / 1000) as i64;
    //    // TODO: verify length of labelset + values as <= 128 UTF8 chars
    //}
}
