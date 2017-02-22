# hayate
A rusty CPS3 arcade board / Hitachi SH2 risc cpu emulator.

So yes, the idea is to code up something that emulates a [CPS3 board](http://wiki.arcadeotaku.com/w/CPS3); The infamously brittle Capcom arcade board from the 90's that powered just a handful arcade fighters (6) before it died a quick death. Part of the reason was because the board was equipped with a 'suicide battery', that would erase some data on the 'game-cart' when it thought people were messing with it. And this happened often, after which arcade owners needed to ship the board back to Capcom.

It took 10 years before the emulation/board modder scene managed to crack the encryption scheme of the game, partly because of this. There are only two active emulators of which I am aware, that support CPS-3: Mame, which I think had the first working implementation, and Final Burn Alpha, which is a port of the Mame code. Both have their issues, so another emulator could have some use.

Info on the board is very sparse. The [Mame code](https://github.com/mamedev/mame/blob/master/src/mame/drivers/cps3.cpp) is just about the only place with useful info I could find, and the comments seem to suggest that the knowledge on this system is far from complete. My plan was to use the Github wiki as a knowledge base. If you have info on the board, please share.

By far the most famous amongst the games that ran on this system is [Street Fighter III](https://en.wikipedia.org/wiki/Street_Fighter_III), which to me is the pinnacle of 2D sprite graphics in games, and which is in general seems like a work of art, made with lots of love. To be honest this project can be seen as a Street Fighter III, Third Strike emulator. I'm unreasonably obsessed with it for someone who doesn't really play the game.

Emulator-wise, there's not much to show for now. I just started. Right now we can just read game data from a zip file, and transform it into a format that we can work with. Just run the built bin with a 'no-cd' game file:

```cargo run <your game file>```

And oh yes! As you can see from the line above, this emulator was coded in Rust, an awesome systems programming language, a-la C++. I guess I'd describe it as C++ weeded from years of cruft. With safety in mind. This emulator is meant as a vehicle to code up a proper sized project. To get things on the way. I got the idea via [yupferris](https://github.com/yupferris), who has been doing very enjoyable streams about Nintendo emulators in Rust.