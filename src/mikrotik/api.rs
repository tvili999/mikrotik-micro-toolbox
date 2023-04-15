use std::{
    io::{Read, Write},
    net::TcpStream,
    vec,
};

#[derive(Debug)]
enum ReadError {
    IoError(std::io::Error),
    Invalid,
    EOF,
}

pub struct RouterOSApi {
    socket: TcpStream,
}

impl RouterOSApi {
    pub fn new(router_ip: &str) -> Result<Self, String> {
        let socket = TcpStream::connect(router_ip)
            .map_err(|e| format!("Couldn't connect to router on '{}': {:?}", router_ip, e))?;

        Ok(Self { socket })
    }

    pub fn login(&mut self, username: &str, password: &str) -> Result<(), String> {
        let repl = self.talk(&[
            "/login",
            format!("=name={}", username).as_str(),
            format!("=password={}", password).as_str(),
        ])?;

        if repl.len() != 0 {
            return Err(format!("RouterOS error: {:?}", repl));
        }

        Ok(())
    }

    pub fn talk(&mut self, command: &[&str]) -> Result<Vec<String>, String> {
        self.write_sentence(command.iter().map(|&word| word.as_bytes()).collect())
            .map_err(|e| format!("Network error: {:?}", e))?;

        let mut res_str: Vec<String> = Vec::new();
        loop {
            for word_raw in self
                .read_sentence()
                .map_err(|e| format!("Network error: {:?}", e))?
                .iter()
            {
                let word = String::from_utf8(word_raw.to_vec()).unwrap();
                if word == "!done" {
                    return Ok(res_str);
                } else {
                    res_str.push(word);
                }
            }
        }
    }

    fn read_sentence(&mut self) -> Result<Vec<Vec<u8>>, ReadError> {
        let mut words: Vec<Vec<u8>> = Vec::new();
        loop {
            let word = self.read_word()?;
            if word.len() == 0 {
                return Ok(words);
            }
            words.push(word);
        }
    }

    fn read_word(&mut self) -> Result<Vec<u8>, ReadError> {
        let len = self.read_len()?;
        if len == 0 {
            return Ok(Vec::new());
        }

        let mut buf: Vec<u8> = vec![0; len];

        let byte_count = self
            .socket
            .read(&mut buf)
            .map_err(|e| ReadError::IoError(e))?;

        if byte_count != len {
            return Err(ReadError::Invalid);
        }

        Ok(buf)
    }

    fn read_bytes_int(&mut self, bytes: u8) -> Result<usize, ReadError> {
        let mut buf: [u8; 1] = [0u8];

        let mut val: usize = 0;
        for _ in 0..bytes {
            let byte_count = self
                .socket
                .read(&mut buf)
                .map_err(|e| ReadError::IoError(e))?;

            if byte_count == 0 {
                return Err(ReadError::EOF);
            }

            val <<= 8;
            val |= buf[0] as usize;
        }
        return Ok(val);
    }

    fn read_len(&mut self) -> Result<usize, ReadError> {
        let mut len = self.read_bytes_int(1)?;

        if (len & 0x80) == 0x00 {
            return Ok(len);
        }

        if (len & 0xC0) == 0x80 {
            len &= !0xC0;
            return Ok((len << 8) | self.read_bytes_int(1)?);
        }

        if (len & 0xE0) == 0xC0 {
            len &= !0xE0;
            return Ok((len << 16) | self.read_bytes_int(2)?);
        }

        if (len & 0xF0) == 0xE0 {
            len &= !0xF0;
            return Ok((len << 24) | self.read_bytes_int(3)?);
        }

        if (len & 0xF8) == 0xF0 {
            return Ok(self.read_bytes_int(4)?);
        }

        return Err(ReadError::Invalid);
    }

    fn write_sentence(&mut self, words: Vec<&[u8]>) -> Result<(), std::io::Error> {
        for w in words.iter() {
            self.write_word(&w)?;
        }
        self.write_word(&[])?;
        Ok(())
    }

    fn write_word(&mut self, buf: &[u8]) -> Result<(), std::io::Error> {
        self.write_len(buf.len())?;
        self.socket.write(buf)?;

        Ok(())
    }

    fn write_len(&mut self, len: usize) -> Result<(), std::io::Error> {
        if len < 0x80 {
            self.socket.write(&[len as u8])?;
        } else if len < 0x4000 {
            let len = len | 0x8000;
            self.socket.write(&[(len >> 8) as u8])?;
            self.socket.write(&[len as u8])?;
        } else if len < 0x200000 {
            let len = len | 0xC00000;
            self.socket.write(&[(len >> 16) as u8])?;
            self.socket.write(&[(len >> 8) as u8])?;
            self.socket.write(&[len as u8])?;
        } else if len < 0x10000000 {
            let len = len | 0xE0000000;
            self.socket.write(&[(len >> 24) as u8])?;
            self.socket.write(&[(len >> 16) as u8])?;
            self.socket.write(&[(len >> 8) as u8])?;
            self.socket.write(&[len as u8])?;
        } else {
            self.socket.write(&[0xF0])?;
            self.socket.write(&[(len >> 24) as u8])?;
            self.socket.write(&[(len >> 16) as u8])?;
            self.socket.write(&[(len >> 8) as u8])?;
            self.socket.write(&[len as u8])?;
        }
        Ok(())
    }
}
