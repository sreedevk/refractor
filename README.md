<h1 align="center">Refractor</h1>
<h5 align="center">pacman mirrorlist manager</h5>
<p align="center">
This is a rust replacement for the <a href="https://xyne.dev/projects/reflector/">Reflector</a> python app for arch linux to retrieve and filter the latest Pacman mirror list.
</p>

This Project is currently under development. [Contributions](#contributions) Welcome.
The following features are currently available:

```bash
  refractor --list-countries # lists pacman mirror count country wise
  refractor --fastest <count> # evaluates & prints  downloaded speed for pacman mirror DBs
```

# Roadmap
```bash
  refractor --fastest n --save /etc/pacman.d/mirrorlist # saves the fastest n mirrors to file
  refractor --sort {age,rate,score,country,delay} # add more sorting options
```

# installation

```bash
  cargo install refractor
```

# contributions
