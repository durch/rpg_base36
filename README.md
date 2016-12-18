[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/durch/rpg_base36/blob/master/LICENCE.md)

Toy extension demonstrating PostgreSQL extensions in Rust, implements base36_encode as found [here](http://big-elephants.com/2015-10/writing-postgres-extensions-part-i).
Initial heavy lifting was done over at [jsoncdc](https://github.com/posix4e/jsoncdc), from which boilerplate code was liberaly ~~stolen~~ borrowed :).

### Testing it out

[![Join the chat at https://gitter.im/durch/rpg_base36](https://badges.gitter.im/durch/rpg_base36.svg)](https://gitter.im/durch/rpg_base36?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

+ Build and start the image:

```bash
docker-compose up
```

+ Bash into it:

```bash
docker exec -it $(docker ps | grep rpgbase | awk '{print $1}') /bin/bash 
```

+ Once inside make and test:
```bash
make install && PGUSER=postgres make installcheck
```

There you go, we now have a fully functional PG extension written in Rust, and glued together with a pinch of C.

Looking forward to any feedback :)
