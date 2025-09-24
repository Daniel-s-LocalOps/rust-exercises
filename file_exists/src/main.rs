use std::path;

trait FileMetadata {
    fn exists(&self) -> bool;
    fn is_writeable(&self) -> bool;
    fn is_readable(&self) -> bool;
}

impl FileMetadata for path::Path {
    fn exists(&self) -> bool {
        self.exists()
    }

    fn is_writeable(&self) -> bool {
        !self.metadata().unwrap().permissions().readonly()
    }

    fn is_readable(&self) -> bool {
        self.metadata().unwrap().permissions().readonly()
    }
}

fn main() {
    let file = path::Path::new("test.txt");

    println!("{:?}", file.exists());
    println!("{:?}", file.is_readable());
    println!("{:?}", file.is_writeable());
}
