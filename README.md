# Mikrotik-micro-toolbox

Mikrotik RouterOS now supports containers, and we can use this to our leverage to manage the routers more easily.

The problem is that some routers have very limited free space, so basically they can't be used with any container, even alpine doesn't fit. 

So the goal of this project is to create a very tiny (<700kB) tool that can do some work for us that we can pack into a really small container, and run it even on very limited routers
