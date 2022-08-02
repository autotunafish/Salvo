# Salvo
A rule variant of Battleship where you have the same number of shots as you have ships.
Create a new cargo bin somewhere, give it a snakecase name like salvo and then replace the 
respective cargo.toml and main.rs. Then from your salvo (or whatever name) directory,
execute cargo run.<br> 
Format the terminal for 10x27 characters for best view, zooming helps!.
The game goes just like Battleship with selecting a position of A through J and 0 through 9.
The results of both yours and the computers turn are displayed.

The computer shot logic will very simply target all the unspent positions surrounding any 
existing hits and save one shot for a random selection. 
