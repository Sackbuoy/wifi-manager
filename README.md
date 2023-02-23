# wifi-manager

Right now the idea is to make a binary named `wifi` and place it in your PATH, then run it with `sudo` assuming you have the line
`%net archlinux=NOPASSWD:/home/cameron/.bin/wifi` 
also requires a `net` group to exist and you to be a member of it.....yeah i dont love this rn

Right now has 3 commands - scan, connect, disconnect
All must be run as root or with the shit above
scan:
`wifi scan`

connect:
`wifi connect -n Whiffie -p mypassword`
optionally with the interface as well
`wifi connect -n Whiffie -p mypassword -i wlan0`

disconnect:
`wifi disconnect`
