use std::{
    fs, // file system
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};
// Listening to the TCP Connection
fn main() {
    

    // listen for TCP connections at the address 127.0.0.1:7878
    // - 127.0.0.1 is an IP address
    // - 7878 is the port
    // - connecting to a port to listen to is known as “binding to a port
    // - unwrap to stop the program if error happens
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();


    // incoming returns an interator with a sequence of streams of type TcpStream
    // A single stream represents an open connection between the client and the server
    // "Connection" means the full request and response process in which
    // - a client connects to the server
    // - the server generates a response
    // - the server closes the connection
    for stream in listener.incoming() {
        // Technically, we are iterating over connection "attempts"
        // The connection might not be successful for a number of reasons, many of them operating system specific. 
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    // next to look at the first line of the HTTP request
    // 1st unwrap handles Option
    // 2nd unwrap handles Result
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

    // if request_line == "GET / HTTP/1.1" {
    //     let status_line = "HTTP/1.1 200 OK";
    //     let contents = fs::read_to_string("hello.html").unwrap();
    //     let length = contents.len();

    //     let response = format!(
    //         "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    //     );

    //     stream.write_all(response.as_bytes()).unwrap();
    // } else {
    //     // some other request
    //     let status_line = "HTTP/1.1 404 NOT FOUND";
    //     let contents = fs::read_to_string("404.html").unwrap();
    //     let length = contents.len();

    //     let response = format!(
    //         "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    //     );

    //     stream.write_all(response.as_bytes()).unwrap();
    // }
}

fn handle_connection_default(mut stream: TcpStream) {
    // adds buffering by managing calls to the std::io::Read trait methods
    let buf_reader = BufReader::new(&stream);
    
    // collect the lines of the request the browser sends to our server
    let http_request: Vec<_> = buf_reader
        .lines() // returns an iterator of Result<String, std::io::Error> by splitting the stream of data whenever it sees a newline byte
        .map(|result| result.unwrap()) // get string from each Result
        .take_while(|line| !line.is_empty()) // take lines until we get a line that is the empty string
        .collect();

    // println!("Request: {http_request:#?}");
    
    // respond with a blank page
    // let response = "HTTP/1.1 200 OK\r\n\r\n";
    // stream.write_all(response.as_bytes()).unwrap();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    // format! to add the file’s contents as the body of the success response
    // To ensure a valid HTTP response, add the Content-Length header, which is set to the size of our response body
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

}

// HTTP Request Format:
//   Method Request-URI HTTP-Version CRLF
//   headers CRLF
//   message-body

// Response Format:
//   HTTP-Version Status-Code Reason-Phrase CRLF
//   headers CRLF
//   message-body


// Request: [
//     "GET / HTTP/1.1",         <- GET (Method)  / (URI) HTTP/1.1 (HTTP-Version)
//     "Host: 127.0.0.1:7878",   <- the rest is headers (and no body)
//     "Connection: keep-alive",
//     "sec-ch-ua: \"Google Chrome\";v=\"131\", \"Chromium\";v=\"131\", \"Not_A Brand\";v=\"24\"",
//     "sec-ch-ua-mobile: ?0",
//     "sec-ch-ua-platform: \"Linux\"",
//     "DNT: 1",
//     "Upgrade-Insecure-Requests: 1",
//     "User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
//     "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
//     "Sec-Fetch-Site: none",
//     "Sec-Fetch-Mode: navigate",
//     "Sec-Fetch-User: ?1",
//     "Sec-Fetch-Dest: document",
//     "Accept-Encoding: gzip, deflate, br, zstd",
//     "Accept-Language: en-GB,en-US;q=0.9,en;q=0.8,ko-KR;q=0.7,ko;q=0.6",
// ]

