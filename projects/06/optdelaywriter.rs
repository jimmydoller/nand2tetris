#[derive(Debug)]
struct OptDelayWriter<'a, W: Write> {
    bufw: BufWriter<W>,
    dbuf: Vec<&'a [u8]>,
    is_write: bool,
}

impl<W: Write> OptDelayWriter<'_, W> {

    pub fn new (stream: W) -> Self {
        OptDelayWriter {
            bufw: BufWriter::new(stream),
            dbuf: Vec::with_capacity(1024),
            is_write: true,
        }
    }

    pub fn write_maybe(&mut self, buf: &'static [u8]) {
        if self.is_write() {
            if !self.dbuf.is_empty() {
                for &x in self.dbuf.iter() {
                    self.bufw.write(&x);
                }
                self.dbuf.clear();
            }

            self.bufw.write(buf);
            self.bufw.flush();

        } else {
            self.dbuf.push(buf);
        }
    }


    pub fn flush(&mut self) {
        self.bufw.flush();
    }

    pub fn is_write(&self) -> bool {
        self.is_write
    }

    pub fn set_write(mut self, b: bool) {
        self.is_write = b;
    }


}
