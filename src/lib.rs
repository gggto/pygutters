use pyo3::prelude::*;

use std::net::{TcpStream, TcpListener};
use interprocess::local_socket::{LocalSocketListener, LocalSocketStream};

use ::gutters::{hail, pick_up, pick_up_and_hail, throw, throw_and_wait, wait};

/// An implemementation of a TCP/IP gutter.
///
/// Instances must be produced through either connect() or accept().
#[pyclass]
struct Gutter {
    stream: TcpStream,
}

#[pymethods]
impl Gutter {
    ///     Connect to a remote TCP/IP gutter.
    ///
    ///     Parameters
    ///     ----------
    ///     address: str
    ///         The address and port of the remote gutter.
    ///
    ///     Returns
    ///     -------
    ///     out: Gutter
    ///         The resulting Gutter.
    #[staticmethod]
    #[pyo3(text_signature = "(address, /)")]
    fn connect(address: &str) -> PyResult<Self> {
        let stream = TcpStream::connect(address)?;
        stream.set_nodelay(true)?;
        Ok(Gutter { stream })
    }

    ///     Accept a remote TCP/IP gutter client connection.
    ///
    ///     Parameters
    ///     ----------
    ///     address: str
    ///         The address to bind.Typically "127.0.0.1:XXXX" where
    ///         XXXX is some port.
    ///     Returns
    ///     -------
    ///     out: Gutter
    ///         The resulting Gutter.
    #[staticmethod]
    #[pyo3(text_signature = "(address, /)")]
    fn accept(address: &str) -> PyResult<Self> {
        let listener = TcpListener::bind(address)?;
        let (stream, _) = listener.accept()?;
        stream.set_nodelay(true)?;
        Ok(Gutter { stream })
    }

    ///     Read a float64 from the remote.
    ///
    ///     Returns
    ///     -------
    ///     out: float
    ///         Float received.
    #[pyo3(text_signature = "($self, /)")]
    fn pick_up(&mut self) -> PyResult<f64> {
        let mut branch = 0.0f64;
        pick_up(&mut self.stream, &mut branch)?;
        Ok(branch)
    }

    ///     Send a float64 to the remote.
    ///
    ///     Parameters
    ///     ----------
    ///     data: float
    ///         Text data to send.
    #[pyo3(text_signature = "($self, branch, /, /)")]
    fn throw(&mut self, branch: f64) -> PyResult<()> {
        throw(&mut self.stream, &branch)?;
        Ok(())
    }

    ///     Read a float64 from the remote and send an acknowledgement.
    ///
    ///     Returns
    ///     -------
    ///     out: float
    ///         Float received.
    #[pyo3(text_signature = "($self, /)")]
    fn pick_up_and_hail(&mut self) -> PyResult<f64> {
        let mut branch = 0.0f64;
        pick_up_and_hail(&mut self.stream, &mut branch)?;
        Ok(branch)
    }

    ///     Send a float64 to the remote and wait for an acknowledgement.
    ///
    ///     Parameters
    ///     ----------
    ///     data: float
    ///         Text data to send.
    #[pyo3(text_signature = "($self, branch, /)")]
    fn throw_and_wait(&mut self, branch: f64) -> PyResult<()> {
        throw_and_wait(&mut self.stream, &branch)?;
        Ok(())
    }

    ///     Send an acknowledgement to the remote.
    #[pyo3(text_signature = "($self, /)")]
    fn hail(&mut self) -> PyResult<()> {
        hail(&mut self.stream)?;
        Ok(())
    }

    ///     Wait for an acknowledgement from the remote.
    #[pyo3(text_signature = "($self, /)")]
    fn wait(&mut self) -> PyResult<()> {
        wait(&mut self.stream)?;
        Ok(())
    }
}

fn name_to_path(name: &str) -> String {
    let path_string = if cfg!(windows) {
        format!(r"\\.\pipe\{}", name)
    } else {
        format!("/tmp/{}", name)
    };
    path_string
}

/// An implemementation of a named pipe duct.
///
/// Instances must be produced through either connect() or accept().
#[pyclass]
struct Duct {
    stream: LocalSocketStream,
}

#[pymethods]
impl Duct {
    ///     Connect to a remote named pipe duct.
    ///
    ///     Parameters
    ///     ----------
    ///     name: str
    ///         The name of the remote duct pipe.
    ///
    ///     Returns
    ///     -------
    ///     out: Duct
    ///         The resulting Duct.
    #[staticmethod]
    #[pyo3(text_signature = "(name, /)")]
    fn connect(name: &str) -> PyResult<Self> {
        let path_string = name_to_path(name);
        let stream = LocalSocketStream::connect(path_string).map_err(|e| anyhow::Error::new(e))?;
        Ok(Duct { stream })
    }

    ///     Accept a remote named pipe duct client connection.
    ///
    ///     Parameters
    ///     ----------
    ///     name: str
    ///         The name of the pipe to bind.
    ///     Returns
    ///     -------
    ///     out: Duct
    ///         The resulting Duct.
    #[staticmethod]
    #[pyo3(text_signature = "(name, /)")]
    fn accept(name: &str) -> PyResult<Self> {
        let path_string = name_to_path(name);
        let listener = LocalSocketListener::bind(path_string).map_err(|e| anyhow::Error::new(e))?;
        let stream = listener.accept().map_err(|e| anyhow::Error::new(e))?;
        Ok(Duct { stream })
    }

    ///     Read a float64 from the remote.
    ///
    ///     Returns
    ///     -------
    ///     out: float
    ///         Float received.
    #[pyo3(text_signature = "($self, /)")]
    fn pick_up(&mut self) -> PyResult<f64> {
        let mut branch = 0.0f64;
        pick_up(&mut self.stream, &mut branch)?;
        Ok(branch)
    }

    ///     Send a float64 to the remote.
    ///
    ///     Parameters
    ///     ----------
    ///     data: float
    ///         Text data to send.
    #[pyo3(text_signature = "($self, branch, /)")]
    fn throw(&mut self, branch: f64) -> PyResult<()> {
        throw(&mut self.stream, &branch)?;
        Ok(())
    }

    ///     Read a float64 from the remote and send an acknowledgement.
    ///
    ///     Returns
    ///     -------
    ///     out: float
    ///         Float received.
    #[pyo3(text_signature = "($self, /)")]
    fn pick_up_and_hail(&mut self) -> PyResult<f64> {
        let mut branch = 0.0f64;
        pick_up_and_hail(&mut self.stream, &mut branch)?;
        Ok(branch)
    }

    ///     Send a float64 to the remote and wait for an acknowledgement.
    ///
    ///     Parameters
    ///     ----------
    ///     data: float
    ///         Text data to send.
    #[pyo3(text_signature = "($self, branch, /)")]
    fn throw_and_wait(&mut self, branch: f64) -> PyResult<()> {
        throw_and_wait(&mut self.stream, &branch)?;
        Ok(())
    }

    ///     Send an acknowledgement to the remote.
    #[pyo3(text_signature = "($self, /)")]
    fn hail(&mut self) -> PyResult<()> {
        hail(&mut self.stream)?;
        Ok(())
    }

    ///     Wait for an acknowledgement from the remote.
    #[pyo3(text_signature = "($self, /)")]
    fn wait(&mut self) -> PyResult<()> {
        wait(&mut self.stream)?;
        Ok(())
    }
}

#[pymodule]
fn gutters(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Gutter>()?;
    m.add_class::<Duct>()?;
    Ok(())
}
