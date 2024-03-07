use scraper::{Html, Selector};





fn keep_selected_chars(input: &str, allowed_chars: &str) -> String 
{
    input.chars()
         .filter(|&c| allowed_chars.contains(c))
         .collect()
}





fn remove_after_char(input: &str, target_char: char) -> String 
{
    let mut parts = input.split(target_char);



        if let Some(part) = parts.next() 
        {
            part.to_string()
        } 
        else
        {
            input.to_string()
        }
}





fn clean_string_and_parse(string_to_send: &str) -> f64
{
    let allowed_chars = "1234567890,%";
    let target_char = '%';

        let raw_string = keep_selected_chars(&string_to_send, allowed_chars);
        let raw_string = remove_after_char(&raw_string, target_char);
        let raw_string: &String = &&raw_string.replace(",", ".");
        let f64_to_send: f64 = raw_string.parse().unwrap();

            return f64_to_send;
}





#[tokio::main]
pub async fn infos() -> (String, f64)
{
    let mut string_to_send: String = String::new();



        // HTML content
        let url = "https://blog.nubank.com.br/taxa-selic/";
        let response = reqwest::get(url).await.expect("Could not load url.");
        let html_content = response.text().await.unwrap();



            // Parse the HTML content
            let document = Html::parse_document(&html_content);
            let p_selector = Selector::parse("p").unwrap();



                // Count, select and copy the paragraph to one string
                let mut paragraph_count = 0;
                for paragraph in document.select(&p_selector) 
                {
                    paragraph_count += 1;

                        if paragraph_count == 10 
                        {
                            string_to_send = paragraph.text().collect::<String>();
                            break;  
                        }
                }


                    // Clean the string and transform it in f64
                    let f64_to_send = clean_string_and_parse(&string_to_send);


                        // Return Values
                        return 
                        (
                            string_to_send,
                            f64_to_send
                        );
}