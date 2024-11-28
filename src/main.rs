use printpdf::*;
use std::fs::File;
use std::io::{self, BufWriter};

fn draw_invoice(
    issuer: &str,
    pay_by: &str,
    recipient: &str,
    items: &[(String, f64, u32)],
    total_amount: f64,
    file_name: &str,
) {
    let (doc, page1, layer1) = PdfDocument::new("Invoice", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    current_layer.use_text("INVOICE", 24.0, Mm(15.0), Mm(280.0), &create_font(&doc, BuiltinFont::HelveticaBold));

    let mut y_position = 260.0;
    for line in vec![
        format!("Issuer: {}", issuer),
        format!("Issued to: {}", recipient),
        format!("Pay By: {}", pay_by),
    ] {
        current_layer.use_text(line, 14.0, Mm(15.0), Mm(y_position), &create_font(&doc, BuiltinFont::Helvetica));
        y_position -= 10.0;
    }

    y_position -= 20.0;
    let header = "Item Name           Rate         Quantity        Total";
    current_layer.use_text(
        header,
        14.0,
        Mm(15.0),
        Mm(y_position),
        &create_font(&doc, BuiltinFont::CourierBold),
    );

    y_position -= 5.0;
    draw_line(&current_layer, Mm(15.0), Mm(y_position), Mm(195.0), Mm(y_position));

    for (name, rate, quantity) in items {
        y_position -= 10.0;
        let total = rate * (*quantity as f64);
        let line = format!(
            "{:<20} ${:<11.2} {:>11} {:>14}",
            name.chars().take(20).collect::<String>(),
            rate,
            quantity,
            format!("${:.2}", total)
        );
        current_layer.use_text(line, 12.0, Mm(15.0), Mm(y_position), &create_font(&doc, BuiltinFont::Courier));
    }

    y_position -= 5.0;
    draw_line(&current_layer, Mm(15.0), Mm(y_position), Mm(195.0), Mm(y_position));

    y_position -= 15.0;
    current_layer.use_text(
        format!("Total Amount: ${:.2}", total_amount),
        14.0,
        Mm(15.0),
        Mm(y_position),
        &create_font(&doc, BuiltinFont::HelveticaBold),
    );

    let file = File::create(file_name).expect("Unable to create file");
    let mut writer = BufWriter::new(file);
    doc.save(&mut writer).expect("Failed to save PDF");
}

fn draw_line(layer: &PdfLayerReference, x1: Mm, y1: Mm, x2: Mm, y2: Mm) {
    let points = vec![(Point::new(x1, y1), false), (Point::new(x2, y2), false)];
    let line = Line {
        points,
        is_closed: false,
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };
    layer.add_shape(line);
}

fn create_font(doc: &PdfDocumentReference, font: BuiltinFont) -> IndirectFontRef {
    doc.add_builtin_font(font).expect("Failed to load font")
}

fn main() -> io::Result<()> {
    let mut issuer = String::new();
    let mut pay_by = String::new();
    let mut recipient = String::new();

    println!("Enter Issuer name: ");
    io::stdin().read_line(&mut issuer)?;
    let issuer = issuer.trim();

    println!("Enter Pay By date (YYYY-MM-DD): ");
    io::stdin().read_line(&mut pay_by)?;
    let pay_by = pay_by.trim();

    println!("Enter name of the person the invoice is issued to: ");
    io::stdin().read_line(&mut recipient)?;
    let recipient = recipient.trim();

    println!("Enter the number of items: ");
    let mut num_items_input = String::new();
    io::stdin().read_line(&mut num_items_input)?;
    let num_items: usize = num_items_input.trim().parse().expect("Invalid number");

    let mut items = Vec::new();
    for i in 0..num_items {
        println!("Enter details for item {}: (name, rate, quantity)", i + 1);

        let mut name = String::new();
        println!("Item name: ");
        io::stdin().read_line(&mut name)?;
        let name = name.trim().to_string();

        let mut rate_input = String::new();
        println!("Rate: ");
        io::stdin().read_line(&mut rate_input)?;
        let rate: f64 = rate_input.trim().parse().expect("Invalid rate");

        let mut quantity_input = String::new();
        println!("Quantity: ");
        io::stdin().read_line(&mut quantity_input)?;
        let quantity: u32 = quantity_input.trim().parse().expect("Invalid quantity");

        items.push((name, rate, quantity));
    }

    let total_amount: f64 = items.iter().map(|(_, rate, qty)| rate * (*qty as f64)).sum();

    let file_name = format!("invoice_{}.pdf", recipient.replace(" ", "_"));
    draw_invoice(issuer, pay_by, recipient, &items, total_amount, &file_name);

    println!("Invoice saved to: {}", file_name);

    Ok(())
}

