#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Point(f32, f32),
    Line(f32, f32, f32, f32),
    Clear,
    Done,
}

impl Command {
    pub fn point<P: Into<(f32, f32)>>(p: P) -> Command {
        let t = p.into();
        Command::Point(t.0, t.1)
    }

    pub fn line_points<P: Into<(f32, f32)>>(
        p1: P,
        p2: P,
    ) -> Command {
        let t1 = p1.into();
        let t2 = p2.into();
        Command::Line(t1.0, t1.1, t2.0, t2.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn line_coords() {
        let l = Command::Line(0.0, 0.0, 1.0, 1.0);
        let json = serde_json::to_string(&l).unwrap();
        let bytes = json.len();
        assert_eq!(json, "{\"Line\":[0.0,0.0,1.0,1.0]}");
        assert_eq!(bytes, 26)
    }

    #[test]
    fn line_points() {
        let l = Command::line_points((0.0, 0.0), (1.0, 1.0));
        let json = serde_json::to_string(&l).unwrap();
        let bytes = json.len();
        assert_eq!(json, "{\"Line\":[0.0,0.0,1.0,1.0]}");
        assert_eq!(bytes, 26)
    }

    #[test]
    fn clear() {
        let c = Command::Clear;
        let json = serde_json::to_string(&c).unwrap();
        let bytes = json.len();
        assert_eq!(json, "\"Clear\"");
        assert_eq!(bytes, 7)
    }

    #[test]
    fn vec() {
        let mut buffer = Vec::new();

        buffer.push(Command::Clear);
        let mut r = 0.0;
        while r < 10.0 {
            let l = Command::Line(0.0, r, 0.0, r + 1.0);
            buffer.push(l);
            r += 1.0;
        }
        buffer.push(Command::Done);

        let json = serde_json::to_string(&buffer).unwrap();
        let bytes = json.len();
        assert_eq!(
            json,
            "[\"Clear\",{\"Line\":[0.0,0.0,0.0,1.0]},{\"Line\":[0.0,1.0,0.0,2.0]},{\"Line\":[0.0,2.0,0.0,3.0]},{\"Line\":[0.0,3.0,0.0,4.0]},{\"Line\":[0.0,4.0,0.0,5.0]},{\"Line\":[0.0,5.0,0.0,6.0]},{\"Line\":[0.0,6.0,0.0,7.0]},{\"Line\":[0.0,7.0,0.0,8.0]},{\"Line\":[0.0,8.0,0.0,9.0]},{\"Line\":[0.0,9.0,0.0,10.0]},\"Done\"]"
        );
        assert_eq!(bytes, 287)
    }
}
