# ⚙️ HTTP server in Rust ([ref](https://www.udemy.com/course/rust-fundamentals/))

This is a basic HTTP/1.1 server implemented in rust language. It is an attempt to learn rust by building a real world application.

## Architecture

Our server will have 3 components:

- TCP listener
  - It provides the ability to listen to TCP connections and read/write bytes from those connections.
- HTTP parser
  - Parses the bytes read by the TCP listener into the HTTP data structure.
- Handler
  - Handles routing logic on the parsed data recieved from the parser.
  - It will execute some code or return files based on the HTTP method and path.
  - It will also create an HTTP response and hand it back to the TCP listener.

## The Server struct

- A `struct` in a programming language is a data type that groups together related data types.
- Values of the `struct` are placed next to each other in memory.
- Implementation block `impl` defines the methods associated with the `struct`.
  - Methods are functions related to the struct.
  - The first parameter of methods is always `self`, which is the instance of the struct that the method is called upon.
- `new()` is considered as a constructor for the struct. It is the general convention. We can call it whatever we want.

### The string slice (&str) and the string (String)

- `&str` is called a string slice.
- It is the immutable reference to a part of the string.
- ```
    let string = String::from("127.0.0.1:8080");
    let string_slice = &string[10..14];

    dbg!(string); // This line will give error "cannot move out of string"
                     Solution: dbg!(&string)
    dbg!(string_slice);
  ```

- String, on the stack looks like this:
  |Name|Value|
  |---|---|
  |length|10|
  |capacity|12|
  |ptr|<address_of_memory_in_heap>|
  - Capacity is the bytes that were allocated to the string.
  - They can dynamically grow or shrink in size.
  - If string grows more than the capacity then a new buffer is allocated to the string with a bigger size and whole string is moved there.
- String is a UTF-8 encoded type.

## HTTP Methods using Enum

- Enumerations `enum`, their values are called _variants_.

  ```
  enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
  }

  let get = Method::GET;
  let post = Method::POST;
  ```

- Simple variants (variants without any value) are represented by 0, 1, 2, 3, ... in memory.

### The optinal query string

- There might be a case when the query string is not present in the request.
- Most languages support null value, the variables can be in one of 2 states, null or not null.
- Trying to use a null value as a not-null value will cause an error, because of this Rust does not support null value.
- To express the absence of value in rust we use rust's standard library's Option enum:
  ```
  pub enum Option<T> {
    None,
    Some(T)
  }
  ```

## Organizing our server components (`mod` and `use`)

- The rust code is organized into modules using `mod` keyword.

  ```
  mod server {
    pub struct Server {
      addr: String,
    }

    impl Server {
      pub fn new(addr: String) -> Self {
          Self {
              addr
          }
      }

      pub fn run(self) {
          println!("Server is running on {}", self.addr);
      }
    }
  }
  ```

- Modules live in a seperate files of their own.
  - `server.rs`
  - `http/mod.rs`
  - `http/request.rs`
  - `http/method.rs`
- We can use the `use` keyword to bring the module's components into the scope.

  ```
  use server::Server;
  use http::Method;
  ```

## Return value of TcpListener

- To listen to tcp connections we need to bind a tcp socket to an address. Rust's standard library provides a `TcpListener` struct to do that.

  ```
  use std::net::TcpListener;
  let listener = TcpListener::bind(&self.addr);
  ```

- The associated function `bind()` returns `Result` enum. This enum is the core of error handling in Rust.
- With `Result` enum, it requires us to acknowledge the possibility of errors beforehand.
- Rust differentiate errors in 2 categories:
  1. Recoverable
  - Example: A File Not Found error.
  2. Unrecoverable
  - Example: Trying to access an index beyond the length of an array.
- Most languages does not distinguish between above 2 errors and handles them with Exception, which is not supportedd by Rust.
  ```
  pub enum Result<T, E> {
    Ok(T),
    Err(E),
  }
  ```

## Check for new connections

- We need a loop to check for new connections on every iteration.
- `accept()` method on `TcpListener` instance accepts a new incoming connection. It returns a `Result` enum which gives a tuple.

  ```
  Result<(TcpStream, SocketAddr)>
  ```

- Tuple is a way to group together values of different types. They are fixed in length.
- The `unwrap()` method terminates the program on error. We want the listener to check for new connection if error occurs. That is, we want our program to continue in case error occured.

  ```
  loop {
    let res = listener.accept();

    if res.is_err() {
      continue;
    }

    let (stream, addr) = res.unwrap();
  }
  ```

- Alternative way of above code.

  ```
  match listener.accept() {
    Ok((stream, _)) => {

    },
    Err(err) => {
      println!("Failed to establish connection {}", err);
      continue;
    }
  }
  ```

## Buffer to Request conversion

- We need to convert the byte array recieved in buffer to the Request struct.
- To do this we need to take help of TryFrom trait.
  - Traits in rust are like interfaces in other languages.
  - TryFrom trait is used for type conversion which may fail in a controlled way.
- `unimplemented!()` macro is used to suppress compiler warnings in case of the function that does not have any definition yet.
- Traits can be used to extend the behaviour of already implemented types.

  ```
  fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
    let string = String::from("value");
    string.encrypt(); // encrypt method is available on String type now.
    unimplemented!()
  }

  trait Encrypt {
    fn encrypt(&self) -> Self {
      unimplemented!()
    }
  }

  impl Encrypt for String {
    fn encrypt(&self) -> Self {
      unimplemented!()
    }
  }
  ```

## Custom Errors and the Error trait

- Display trait is used when we are formatting the string.

  ```
  println!("Recieved a request: {}", String::from_utf8_lossy(&buffer));

  // Here the String needs to implement the Display trait.
  // Whatever we pass as second parameter in println!() macro has to
  // implement Display trait

  // {:?} invokes the implementation of the Debug trait
  ```

- `?` operator is used as a shorthand for the following:

  ```
  match str::from_utf8(buf) {
      Ok(request) => {},
      Err(err) => return Err(ParseError::InvalidEncoding)
  }

  let result = str::from_utf8(buf)?;
  // This returns the error value from the whole function
  // The error value goes through the from() function of the From trait.
  ```

## Iterating over the request string slice

- This returns an iterator:
  ```
  request.chars()
  ```
- Iterator gives you the functionality of going through the values of collection one by one. They come with `next()` method.
- `next()` method returns an `Option<Item>` which will be `None` once all the values are gone through otherwise `Some(Item)`

  ```
  // GET /search?name=abc&sort=1 HTTP/1.1
  fn get_next_word(request: &str) -> Option<(&str, &str)> {
      for (i, c) in request.chars().enumerate() {
          if c == ' ' {
              return Some((&request[..i], &request[i + 1..]));
          }
      }
      None
  }
  ```

- The function above goes through the request string slice and return method and request string slices.
  - In case of None, we need our `Option` to be converted to `Result`.
- `ok_or()` is a method on `Option` enum, which transforms `Option` to `Result`
- Reusing the name of the local variable is called _variable shadowing_.

## Lifetimes

- Whenever the request comes to the server it is in the buffer whose values are then stored in the object below

```
  pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
  }
```

Here, the `path` and the `query_string` is of type `String` and `Option<String>` which refers
to the data kept on heap. In this case the data is copied from buffer and then kept on heap.

- Instead we can use string slice to reference the string on the buffer, the object will look like the following,

```
  pub struct Request {
    path: &str,
    query_string: Option<&str>,
    method: Method,
  }
```

Using the object above, compiler complains about the missing lifetime specifier.

- What if the buffer, that the `Request` object is pointing to, is deallocated.
  - This scenario is called dangling reference or use after free.
  - We can say that the `Request` has a longer lifetime than the buffer.
- Lifetime ensures that the references are valid as long as we need them to be.
  - Scope for which a reference is valid.
- Specifying a lifetime allows us to communicate to a compiler that some references are 'related' and are expected to share the same lifetime.

## Query string representation using HashMap

- `HashMap` is one of the collection provided by the Rust's standard library

```
  pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
  }
```

## Copy and Clone Types

- Simple integer values implements the `Copy` trait.
  - The `Copy` trait tells that the value can be copied bit by bit.
- But in case it is complex value whose reference is stored on stack (for example: string)
  - Then it implements `Clone` trait.

```
#[derive(Clone, Copy)]
pub enum StatusCode {
  Ok = 200,
  BadRequest = 400,
  NotFound = 404
}
```

## Dynamic vs Static Dispatch

- When passing traits as an argument to the function the compiler has to figure out which implementation of the trait's function has to be called.

  - To do this we use `dyn`.

  ```
  pub fn send(&self, stream: &mut dyn Write) -> IoResult<()>
  ```

- Dynamic dispatch: Concrete implementation of function will be resolved at runtime.

  - vtable maintains a mapping.
  - At runtime, rust refers the vtable for the function.

- Static Dispatch:
  - In case of this dispatch compiler creates multiple functions with possible values that the trait argument may hold.
  - This creates a larger binary and also the compile time increases.
  - This reduces the runtime costs because there is no need to maintain vtable.

## Making a Handler trait

- We need a handler to handle the responses to the request.

```
pub trait Handler {
  fn handle_request(&mut self, request: &Request) -> Response;

  fn handle_bad_request(&mut self, e: &ParseError) -> Response;
}
```

-
