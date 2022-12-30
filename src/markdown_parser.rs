pub mod markdown {
    use gtk::prelude::*;
    pub fn find_headers(buffer: &gtk::TextBuffer) {
        // text content
        let text_content = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false);

        // vevtor of buffer per line
        let linevec: Vec<&str> = text_content.split('\n').collect();

        // iterator integer
        let mut c = 0;

        for x in linevec.iter() {
            c += 1;

            let style = match (
                x.chars().nth(0),
                x.chars().nth(1),
                x.chars().nth(2),
                x.chars().nth(3),
                x.chars().nth(4),
            ) {
                // h5 -> h1
                (Some('#'), Some('#'), Some('#'), Some('#'), Some('#')) => "Header_5",
                (Some('#'), Some('#'), Some('#'), Some('#'), _) => "Header_4",
                (Some('#'), Some('#'), Some('#'), _, _) => "Header_3",
                (Some('#'), Some('#'), _, _, _) => "Header_2",
                (Some('#'), _, _, _, _) => "Header_1",

                // italitcs, bold, ib
                (Some('*'), Some('*'), Some('*'), _, _) => "italic",
                (Some('*'), Some('*'), _, _, _) => "bold",
                (Some('*'), _, _, _, _) => "bi",

                // lists
                (Some('-'), Some('-'), Some('-'), Some('-'), Some('-')) => "l5",
                (Some('-'), Some('-'), Some('-'), Some('-'), _) => "l4",
                (Some('-'), Some('-'), Some('-'), _, _) => "l3",
                (Some('-'), Some('-'), _, _, _) => "l2",
                (Some('-'), _, _, _, _) => "l1",

                // blocks
                (Some('>'), Some('>'), Some('>'), Some('>'), Some('>')) => "b5",
                (Some('>'), Some('>'), Some('>'), Some('>'), _) => "b4",
                (Some('>'), Some('>'), Some('>'), _, _) => "b3",
                (Some('>'), Some('>'), _, _, _) => "b2",
                (Some('>'), _, _, _, _) => "b1",

                // no formatting
                (_, _, _, _, _) => "none",
            };
            if style != "none" {
                buffer.remove_all_tags(
                    &buffer.iter_at_line(c - 1).unwrap(),
                    &buffer
                        .iter_at_line_offset(c - 1, x.chars().count() as i32)
                        .unwrap(),
                );

                // once names are figured out, "Header_1" will be replaced wiith the output of the above match ^
                buffer.apply_tag_by_name(
                    style,
                    &buffer.iter_at_line(c - 1).unwrap(),
                    &buffer
                        .iter_at_line_offset(c - 1, x.chars().count() as i32)
                        .unwrap(),
                );
            };
        }
    }
}
