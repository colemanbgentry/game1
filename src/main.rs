use std::io;
//This is the main function to start the game loop.
fn main() {
    introduction();
    exit_room();
}
//Just something that allows the player to know what is going on in the beginning. 
fn introduction() {
    println!("You are now in a small room. A bright ceiling light glows above you.");
    println!("As you examine ahead of you, you notice a door. On the right of this door, you see a sign.");
    println!("The sign reads,\"Figure out the four digit code, enter into the keypad, then you will be released.\"");
}
//The main room that the player will be in. 
fn exit_room() {
    exit_room_options(0);
    let mut found_out = 0;
    let mut checked =  1;
    loop { // Loops were interesting to learn about not like a for loop or a while loop that just loop will break; is reached. 
        println!("Enter a number to choose what to do next.");
        let mut choice = String::new();
        io::stdin() // The stdin readline was also new to learn.
            .read_line(&mut choice)
            .expect("\"What do I want to do?\", you think? Enter a number from list of options.");

        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue // This might not even work but just in case, its for making sure the player uses a number. 
        };
        if choice == 1 {
            if checked == 1 {
                examine_exit_room();
                checked = checked + 1;
                found_out = found_out + 1;
            } else {
                println!("You have already examined this room.")
            }
        } else if choice == 2 {
            door();
        } else if choice == 3 {
            sit_down();
        } else if choice == 4 {
            if found_out == 1 {
                exit_room_options(found_out)
            } else {
                exit_room_options(found_out);
            }
        } else if choice == 5 && found_out == 1 {
            sign()
        }
        else {
            println!("\"What do I want to do?\", you think? Enter a number from list of options.")
        }
    }
}
// A way for the player to learn options then more options.
fn exit_room_options(x: i32) {
    println!("What do you want to do?");
    println!("1. Examine the room again?");
    println!("2. Go to keypad?");
    println!("3. Sit down?");
    println!("4. Options?");
    if x == 1 {
        println!("5. Examine the sign?")
    }
}
//More dialog
fn examine_exit_room() {
    println!("You wonder around the room and you notice something.");
    println!("You notice that the sign is slightly curled at one end.");
    println!("You now have another option!");
}
//Funny bit of dialog
fn sit_down() {
    println!("You decide to sit down and wait.");
    println!("And wait");
    println!("And wait even more");
    println!("And wait even even even more");
    println!("Then you decide that waiting wasn't what you should be doing.");
    println!("You get back up on your feet.")
}
//A clue dialog
fn sign() {
    println!("You approuch the sign and lift up the corner.");
    println!("You then find a list of numbers.");
    println!("It reads,
            \"5:4\" 
            \"3:8\"
            \"0:0\"
            \"2:2\"
            \"1:8\"
            \"4:9\"
            \"9:4\"
            \"8:1\"
            \"4:3\"
            \"6:2\"
            \"9:9\"");
    println!("Not sure what it all means, but you accept the clue. You will remember these numbers.")
}
//The final piece of the game the exit. 
fn door() {
    door_options();
    loop {
        println!("What do you want to do");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("\"What do I want to do?\", you think to yourself? 
                           Enter a number from list of options.");

        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        if choice == 1 {
            enter_password();
            println!("You lower your hand from the keypad.")
        } else if choice == 2 {
            println!("You don't feel ready to enter a password yet. You want to see if you can find a clue.");
            break;
        } else {
            println!("\"What do I want to do?\", you think? Enter a number from list of options.")
        }
    }
}
//More dialog for the door. 
fn door_options() {
    println!("You approuch the door.
    A panel pops out of the wall and a keypad rests on the end, 
    the keypad arranged like a telephone, 1 to 9 with a 0 at the bottom.
    A screen blips to life beside the keypad, it reads,
    \"Enter the four digit password\"
                [ , , , ]
    ");
    println!("1. Enter a password?");
    println!("2. Not ready yet?");
}
//Get one number wrong won't allow you through.
fn enter_password() {
    let first_number = 0;
    let second_number = 8;
    let third_number = 2;
    let fourth_number:i32 = 8;
    let mut incorrect: i32 = 0;
    println!("You lift your hand up to the keypad.");
    println!("You want to do... ");
    let first: i32 = press_button();
    let second: i32 = press_button();
    let third: i32 = press_button();
    let fourth: i32 = press_button();
    if first != first_number {
        incorrect = incorrect + 1;
    }
    if second != second_number {
        incorrect = incorrect + 1;
    }
    if third != third_number {
        incorrect = incorrect + 1;
    }
    if fourth != fourth_number {
        incorrect = incorrect + 1;
    }
    if incorrect > 0 {
        println!("With all four number in their places, the screen goes blank, then flashes red.
         A horn blurs for a second, you have a feeling didn't get it correct.")
    } else {
        println!("With all four number in their places, the screen goes blank, then flashes green.
        A chime sounds and the door slowly opens as the hydrolics push it open and you leave bright room.
        You found yourself in a forest, you look around and find nothing around you even the room you came
        out of. The voice can still be heard though.
        Congradulations, till next time....")
    }

}
//This was fun. Allow me to use the same function to be called and get a number many times.
fn press_button() -> i32 {
    loop {
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("\"What do I want to do?\", you think to yourself? 
                           Enter a number from list of options.");

        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        if choice > 9 {
            println!("What the? I can only do 1 to 9, you think to yourself?")
        }else {
            button_interactions(choice);
            return choice;
        }
    }
}
//A bit of cool dialog using the match system, which isn't like your normal switch.  
fn button_interactions(x: i32) {
    match x {
        0 => println!("You lower your finger down to the zero and press it."),
        1 => println!("You raise your finger up to the one and press it."),
        2 => println!("You raise your finger up to the two and press it."),
        3 => println!("You raise your finger up to the three and press it."),
        4 => println!("You move your finger to the left to the four and press it."),
        5 => println!("Your finger goes down to the five and press it."),
        6 => println!("You move your finger to the right to the six and press it."),
        7 => println!("You lower your finger down to the seven and press it."),
        8 => println!("You lower your finger down to the eight and press it."),
        9 => println!("You lower your finger down to the nine and press it."),
        _ => println!("I'm not sure how you got here")
    }
}