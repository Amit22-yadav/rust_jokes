
/// This function selects a random joke from a predefined list of jokes.
pub fn get_random_joke() -> &'static str {
    let jokes = [
        "Why do Rustaceans never panic? Because they always handle their errors!",
        "What did the Rust programmer say at the party? 'I'm borrowing happiness immutably.'",
        "Why do Rust programmers love Ferris? Because he knows how to lifetimes.",
        "How does a Rust developer break up with their partner?'I'm sorry, but I can't share ownership of this relationship anymore.",
        "Rust programmers have the best relationships.They know how to manage lifetimes like pros!",
        "In C we had to code our own bugs. In C++ we can inherit them.",
        "Why don't Rustaceans need garbage collectors? Because they take out their own trash with drop.",
        "Why did the Rust compiler join the marathon? To prove it's fast enough... but it always insists on double-checking everything first!",
        "What do Rust developers do at stand-ups? They just borrow time and return ownership at the end.",
        "Why do Rust programmers love constants? Because change is hard!",
        "Why don't Rust programmers need wallets? Because they never own anything, they just borrow it!",
        "C++ walks into a bar and forgets to pay its tab. Rust walks in and drops its ownership at the door.",
        "In C++, you delete your memory manually. In Rust, the memory deletes you if you misuse it.",
        "C++ lets you have null pointers and then blames you when you dereference them. Rust just says, 'Option or nothing!",
        "C++: 'I'll let you shoot yourself in the foot.'Rust: 'Let me check your gun, your foot, and your intent first.",
        "In C++, threads are like toddlers with scissors. In Rust, they're toddlers in padded rooms with safety scissors",
        "C++ promises you multithreading. Rust promises you multithreading without tears.",
        "Rust's compiler holds your hand. C++'s compiler slaps it away and says, 'Figure it out!",
        "C++ makes you memorize everything about the language to avoid pitfalls. Rust just gives you lifelines and says, 'Go be productive.",
        "Learning C++ is like learning to juggle knives blindfolded. Learning Rust is like learning to juggle—with safety nets and foam knives.",
        "Rust beginners: This should work! Compiler: Nope, ownership rules violated.",
        "Rust programmers on a date: Can I borrow your heart… immutably?",
        "Rustacean: Are you looking for a long-term relationship? 
        GF: Of course!
        Rustacean: Good, because I can't handle dangling references.",
        "GF: So, what do you think of us?
        Rustacean: I think we’re an Ok() with no Err() so far!",
        "Friend 1: How do you know if someone’s a true friend?
        Friend 2: If they don’t panic when you unwrap() your problems.",
        "Friend 1: Do you think we’ll stay friends forever?
        Friend 2: As long as our lifetimes don’t 'end, we’re good!",
        "Our friendship is like Rc: shared ownership, zero drops.",
        "In Rust and friendship, lifetimes matter.",
        "Our bond is immutable—arguments can’t change us.",
        "Friends are the Ok() to my Result.",
        "I tried to borrow your heart, but the borrow checker said it’s already owned.",
        "Dating me is like debugging in Rust: frustrating at first, but so worth it when it works.",
        "You make my heart race like an infinite loop without a break.",
        "C++ developers: 'Memory management is easy!' Rust: 'Hold my borrow checker.'",
        "C++ is like a haunted house—enter at your own risk, and don’t touch anything.",
        "C++: ‘Oops, I double-freed memory.’ Rust: ‘What is this madness?’",
        "Why debug in C++ when you can just rewrite the program in Rust?",
        "Rust is like, ‘Let’s use lifetimes to make sure we’re not breaking up,’ and C++ is like, ‘I’m fine with null relationships.’",
        "Rust always wants to make their relationship type-safe, but C++ insists on using void.",
        "C++ is always messing around with pointers, and Rust is there to make sure nothing gets dereferenced without permission.",
    ];
    let index = rand::random::<usize>() % jokes.len();
    jokes[index]
}

