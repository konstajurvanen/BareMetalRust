# BareMetalRust (Challenge #3)
The solution for Challenge #3: Application Lifecycle of the course "COMP.530-01-2021-2022-1, Bare Metal Rust" in Tampere University.

## Explanation of the application:
  
The application includes an infinite loop that displays text on the lcd-display of Longan Nano. The text content and position is defined by a static mutable variable, the value of which is modified in the interrupt handler for the TIMER0_UP -interrupt. Before the main loop of the program, a global timer is initialized to trigger the TIMER0_UP -interrupt with frequency of 2Hz. The main loop sets the system into sleep-mode after displaying the changes caused by the latest interrupt, until the next interrupt is triggered. Therefore, the content on the lcd-display changes twice a second. 

## Result:
The program makes the Longan Nano device very sleepy and it ends up snoozing endlessly. It wakes up after every eighth interrupt (shown text: o . o), but the "z"s start to appear again after it closes its eyes (o . o --> - . -).
