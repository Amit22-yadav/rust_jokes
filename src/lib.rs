
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
    ];
    let index = rand::random::<usize>() % jokes.len();
    jokes[index]
}

