# Hangman

This is a first attempt to dip my toes into the Rust language.  
The goals of this project are to 
1. Explore the borrowing system in rust
2. Create a comprehensive test suite using the standard testing library rust provides
3. Create a fun(ctional) game

## Implementation
The secret word generator pulls from a predefined list of words stored locally. The function
uses [resovoir sampling](https://youtu.be/A1iwzSew5QY) so that the file didn't needed to be stored
in memory or iterated over twice.  Generating random number is probably not the fastest thing but it was easier than dealing with borrow checker for the other solutions I thought of.

## Summary
I didn't run into as many borrow checker errors as I expected since I didn't end up doing anything too
crazy.  Oh well, next time I'll get wild. I feel like I am following the path of everyone else where the first couple weeks are dominated
by borrow-checker errors.  A couple things still puzzle me...
* There seems to be many instances of return types of `Option<Result<_,_>>`, is there an idiomatic way of getting th data out other than `.unwrap().unwrap()`?
* I've been using the Spock testing framework and have probably gotten lazy, but I feel like mocking 
services or resources here would be pretty painful in a large project.  Most likely this is a lack of experience with multiple testing patterns on my part.
