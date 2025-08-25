use std::process::{Command, Stdio};
use std::time::Duration;
use fantoccini::{ClientBuilder, Locator};
use fantoccini::wd::Capabilities;





pub static mut PREVENT_KILL: bool = false;





fn clean_string_from_vector(vector_of_string: Vec<String>) -> Vec<String>
{   
    let mut vector_of_clean_strings = Vec::new();

    for string in vector_of_string
    {
        let mut separeted_string = string.split_whitespace().collect::<Vec<&str>>();
        if separeted_string.len() == 7
        {
            for _ in 0..3 { separeted_string.remove(2); }
            separeted_string.remove(3);
        }

        if separeted_string.len() == 10
        {
            for _ in 0..7 { separeted_string.remove(2); }
        }

        let mut final_string = String::new();
        for string in separeted_string { final_string.push_str( &format!("{}            ", string) ); }

        vector_of_clean_strings.push(final_string);
    }

    vector_of_clean_strings
}



#[allow(static_mut_refs)]
#[tokio::main]
pub async fn infos() -> Vec<String>
//-> (Vec<String>, f64)
{
    // Prevent the app to be close while starting de webdriver, to prevent memory leaks
    unsafe{PREVENT_KILL = true};

    // Start the Geckodriver WebDriver
    let web_driver = Command::new("geckodriver").stdout(Stdio::null()).stderr(Stdio::null()).spawn();
    std::thread::sleep(Duration::from_millis(500));

    // Start the Fantoccini client
    let cap: Capabilities = serde_json::from_str(r#"{"moz:firefoxOptions":{"args":["--headless"]}}"#,).unwrap();
    let client = ClientBuilder::native().capabilities(cap).connect("http://localhost:4444").await.unwrap();

    // Connect the Fantoccini client to the site of brazil governament that has the Selic tax values
    client.goto("https://www.bcb.gov.br/controleinflacao/historicotaxasjuros").await.unwrap();

    // Get the last 6 Selic tax value
    let list_lenght: u8 = 13;
    let mut list_elements: Vec<String> = Vec::new();
    for index in 1..(list_lenght + 1)
    {
        let search = format!("/html/body/app-root/app-root/div/div/main/dynamic-comp/div/div/bcb-histtaxajuros/div[1]/table/tbody/tr[{}]", index);
        let search_result = client.wait().for_element(Locator::XPath(&search)).await.unwrap().text().await.unwrap();
        list_elements.push(search_result);
    }

    // Close the Client and the Geckodriver WebDriver
    client.close().await.unwrap();
    web_driver.unwrap().kill().unwrap();
    unsafe{PREVENT_KILL = false};

    clean_string_from_vector(list_elements.clone())
}
