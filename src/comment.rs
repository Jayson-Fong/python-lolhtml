use lol_html::html_content::Comment as NativeComment;
use pyo3::prelude::*;

use crate::{ContentType, NativeRefWrap, IntoNative};

#[pyclass(unsendable)]
pub struct Comment {
    inner: NativeRefWrap<NativeComment<'static>>,
}

impl Comment {
    pub(crate) fn from_native_mut(c: &mut NativeComment<'_>) -> Self {
        Self {
            inner: unsafe { NativeRefWrap::wrap(c) },
        }
    }
}

#[pymethods]
impl Comment {
    #[getter]
    pub fn text(&self) -> PyResult<String> {
        self.inner
            .get()
            .map(|c| c.text().to_string())
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    #[setter]
    pub fn set_text(&mut self, text: &str) -> PyResult<()> {
        self.inner
            .get_mut()?
            .set_text(text)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(())
    }

    pub fn remove(&mut self) -> PyResult<()> {
        self.inner.get_mut()?.remove();
        Ok(())
    }

    #[getter]
    pub fn removed(&self) -> PyResult<bool> {
        Ok(self.inner.get()?.removed())
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

    pub fn __repr__(&self) -> PyResult<String> {
        let text = self.text()?;
        Ok(format!("Comment(text={:?})", text))
    }
}
