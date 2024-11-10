use std::fs::File;
use std::io::{self, Write};

fn draw_invoice(issuer: &str, pay_by: &str, recipient: &str, amount: &str) -> String {
    let mut invoice = String::new();
      invoice.push_str("+------------------------------------------------+\n");
    invoice.push_str("|               INVOICE                          |\n");
    invoice.push_str("|                                                |\n");

    let issuer_line = format!("| Issuer: {:<40}|\n", issuer);
       let recipient_line = format!("| Issued to: {:<37}|\n", recipient);
     let pay_by_line = format!("| Pay By: {:<38}|\n", pay_by);
    let amount_line = format!("| Total Amount: ${:<32}|\n", amount);

        invoice.push_str(&issuer_line);
            invoice.push_str(&recipient_line);
           invoice.push_str(&pay_by_line);
    invoice.push_str(&amount_line);
        invoice.push_str("|                                                |\n");
    invoice.push_str("+------------------------------------------------+\n");
    invoice
} // Lol this part was a bit complicated. So a bit of backstory, I have been recently working on a similar project of mine called Welp! 
// You can basically see it at https://welp.it.com. I had been working on that project side by side so after completing chapter 11, I got the idea to make this.

fn main() -> io::Result<()> {
    let mut issuer = String::new();
      let mut pay_by = String::new();
    let mut recipient = String::new();
    let mut amount = String::new();

    println!("Enter Issuer name: ");
    io::stdin().read_line(&mut issuer)?;
    let issuer = issuer.trim();

    println!("Enter Pay By date (YYYY-MM-DD): ");
    io::stdin().read_line(&mut pay_by)?;
      let pay_by = pay_by.trim();

    println!("Enter name of the person the invoice is issued to: ");
    io::stdin().read_line(&mut recipient)?;
    let recipient = recipient.trim();

        println!("Enter Total Amount: ");
        io::stdin().read_line(&mut amount)?;
        let amount = amount.trim();
    let invoice_content = draw_invoice(issuer, pay_by, recipient, amount);
    println!("{}", invoice_content);

    let file_name = format!("invoice_{}.txt", recipient.replace(" ", "_"));
    let mut file = File::create(&file_name)?;

        file.write_all(invoice_content.as_bytes())?;

    println!("Invoice saved to: {}", file_name);

    Ok(())
}
