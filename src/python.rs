extern crate cpython;

use crate::cpython::PyClone;
use cpython::{ToPyObject, PythonObject, PyResult, py_module_initializer, py_class};
use std::cell::RefCell;
use cpython::{PyList, PyDict, PyInt, PyString};

mod cpt;
use cpt::data_types::data_types::{DataTypes, SimilarityScores, SequenceAttributes};
use cpt::data_types::data_types::SequenceMatchFunction;

impl ToPyObject for DataTypes{
    type ObjectType = PyInt;
    fn to_py_object(&self, py: cpython::Python<'_>) -> <Self as cpython::ToPyObject>::ObjectType {
        match self {
            Self::Integer(value) => value.to_py_object(py),
            Self::U8(value) => value.to_py_object(py),
        }
    }
}

impl ToPyObject for SequenceMatchFunction{
    type ObjectType = PyString;
    fn to_py_object(&self, py: cpython::Python<'_>) -> <Self as cpython::ToPyObject>::ObjectType {
        match self {
            // StrictEqual,
            SequenceMatchFunction::StrictEqual => {
                PyString::new(py, "StrictEqual")
            }
            // AlgebraicDistance,
            SequenceMatchFunction::AlgebraicDistance => {
                PyString::new(py, "AlgebraicDistance")
            }
            // SequenceLength
            SequenceMatchFunction::SequenceLength => {
                PyString::new(py, "SequenceLength")
            }
        }
    }
}

impl ToPyObject for SimilarityScores{
    type ObjectType = PyDict;
    fn to_py_object(&self, py: cpython::Python<'_>) -> <Self as cpython::ToPyObject>::ObjectType {
        let sim_score = PyDict::new(py);
        match self {
            // Similarity(f32),
            SimilarityScores::Similarity(value) => {
                // sim_score.set_item(py, "metric", "Similarity");
                sim_score.set_item(py, "value", value.to_py_object(py));
            }
            // Distance(f32),
            SimilarityScores::Distance(value) => {
                // sim_score.set_item(py, "metric", "Distance");
                sim_score.set_item(py, "value", value.to_py_object(py));
            }
            // Length(usize),
            SimilarityScores::Length(value) => {
                // sim_score.set_item(py, "metric", "Length");
                sim_score.set_item(py, "value", value.to_py_object(py));
            }
            // IsEqual(bool),
            SimilarityScores::IsEqual(value) => {
                // sim_score.set_item(py, "metric", "IsEqual");
                sim_score.set_item(py, "value", value.to_py_object(py));
            }
            // None
            SimilarityScores::None => {}
        }
        sim_score
    }
}

py_module_initializer!(cpt_rust, cpt_rustinit, PyInit_cpt_rust, |py, m| {
    m.add(py, "__doc__", "Module documentation string")?;
    m.add_class::<CPT>(py)?;
    println!("Hello World!");
    Ok(())
});

py_class!(class CPT |py| {
    data tree: RefCell<cpt::cpt::CPT<DataTypes>>;
    def __new__(_cls) -> PyResult<CPT> {
        let cpt = cpt::cpt::CPT::<DataTypes>::new();
        CPT::create_instance(py, RefCell::new(cpt))
    }
    def train(&self, sequence: PyList) -> PyResult<usize> {
        self.tree(py).borrow_mut().add_sequence_to_root(
            sequence.iter(py).map(|item|{
                DataTypes::Integer(item.extract(py).unwrap())
            }).collect()
        , None);
        Ok(0)
    }

    def match_sequence(&self, input_seq_py: PyList) -> PyResult<PyList> {
        let sequence = input_seq_py.iter(py).map(|item|{
            DataTypes::Integer(item.extract(py).unwrap())
        }).collect::<Vec<crate::cpt::data_types::data_types::DataTypes>>();

        let prediction = self.tree(py).borrow().match_sequence(
            &sequence,
            false,
            &[SequenceMatchFunction::SequenceLength, SequenceMatchFunction::AlgebraicDistance],
        );
        // ([(1, [(SequenceLength, Length(1)), (AlgebraicDistance, Distance(0.0))]), (5, [(SequenceLength, Length(1)), (AlgebraicDistance, Distance(3.0))]), (6, [(SequenceLength, Length(1)), (AlgebraicDistance, Distance(4.0))])], {AlgebraicDistance: Distance(7.0), SequenceLength: Length(3)})
        // ([(1, [(SequenceLength, Length(1)), (AlgebraicDistance, Distance(0.0))]), (2, [(SequenceLength, Length(1)), (AlgebraicDistance, Distance(0.0))]), (3, [(SequenceLength, Length(1)), (AlgebraicDistance, Distance(1.0))])], {AlgebraicDistance: Distance(1.0), SequenceLength: Length(3)})
        // The result is a list of matched sequences and scores
        let py_results = PyList::new(py, &[] );
        prediction.into_iter().for_each( |matched_seq_tot_scores| {
                let py_result = PyDict::new(py);
                let matched_score = matched_seq_tot_scores.1.to_py_object(py); 
                let matched_seq = PyList::new(py, &[] );
                matched_seq_tot_scores.0.into_iter().for_each(|matched_node|{
                    let node_id = matched_node.0;
                    matched_seq.append(py,
                        self.tree(py).borrow().get_data(node_id).unwrap().to_py_object(py).as_object().clone_ref(py)
                    );
                });

                py_result.set_item(py, "sequence",  matched_seq.as_object().clone_ref(py));
                py_result.set_item(py, "score",  matched_score.as_object().clone_ref(py));
                
                py_results.append(py, py_result.as_object().clone_ref(py) );
            }
        );
        Ok(py_results)
    }
    def to_dot(&self) -> PyResult<PyString> {
        Ok(PyString::new(py, &self.tree(py).borrow().to_dot()))
    }
});
