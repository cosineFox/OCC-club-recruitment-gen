use qrcode::QrCode;

#[derive(Debug, Clone)]
pub struct AppState {
    pub url: String,
    pub qr_matrix: Option<Vec<Vec<bool>>>,
    pub should_quit: bool,
}

impl AppState {
    pub fn new(url: String) -> Self {
        Self {
            url,
            qr_matrix: None,
            should_quit: false,
        }
    }

    pub fn generate_qr(&mut self) {
        if let Ok(code) = QrCode::new(self.url.as_bytes()) {
            let size = code.width();
            let qr = code.into_colors();
            let mut matrix = vec![vec![false; size]; size];

            for (i, color) in qr.iter().enumerate() {
                let x = i % size;
                let y = i / size;
                matrix[y][x] = *color == qrcode::Color::Dark;
            }

            self.qr_matrix = Some(matrix);
        }
    }
}