use lol_html::html_content::EndTag as NativeEndTag;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use crate::{ContentType, HasNative, IntoNative, NativeRefWrap};

#[pyclass(unsendable)]
pub struct EndTag {
    inner: NativeRefWrap<NativeEndTag<'static>>,
}

impl HasNative for EndTag {
    type Native = NativeEndTag<'static>;
}

#[pymethods]
impl EndTag {
    #[getter]
    pub fn name(&self) -> PyResult<String> {
        self.inner
            .get()
            .map(|e| e.name())
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }

    #[setter]
    pub fn set_name(&mut self, name: &str) -> PyResult<()> {
        self.inner.get_mut()?.set_name_str(name.to_string());
        Ok(())
    }

    #[pyo3(signature = (content, content_type=None))]
    pub fn before(
        &mut self,
        content: &str,
        content_type: Option<ContentType>,
    ) -> PyResult<()> {
        self.inner
            .get_mut()?
            .before(content, content_type.into_native());
        Ok(())
    }

    #[pyo3(signature = (content, content_type=None))]
    pub fn after(
        &mut self,
        content: &str,
        content_type: Option<ContentType>,
    ) -> PyResult<()> {
        self.inner
            .get_mut()?
            .after(content, content_type.into_native());
        Ok(())
    }

    pub fn remove(&mut self) -> PyResult<()> {
        self.inner.get_mut()?.remove();
        Ok(())
    }
}
