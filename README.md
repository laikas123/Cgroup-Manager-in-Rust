<h3>About</h3>

This project is a command line tool designed to help simplify managing v2 cgroups.
It has been tested on Ubuntu 22.04, where cgroups have been enabled by default.
For a better understanding of how cgroups work, what controllers are, etc.
I recommend the following <a href="https://docs.kernel.org/admin-guide/cgroup-v2.html">tutorial</a>
which is very helfpul.



<h3>Setting up your environment</h3>

Environment setup is pretty minimal to use this project. There are only two global settings
that need to be configured. They are the root cgroup directory which by default is 
/sys/fs/cgroup on Ubuntu 22.04, and the second is your username which by default is mine,
logan. In the globals.rs file these can be set as seen below.

<br>
<br>
<img src="https://drive.google.com/uc?export=view&id=1xIG15rMXinCONXqzJQEepkrWdXFevU1Q">


These are used by the program as follows:

<strong>CGROUPROOT</strong> is used to locate the root cgroup folder. This is the parent folder
(or root node in terms of cgroup hierarchy) where children cgroup folders will be created. The 
tool uses this location to create directories that will correspond to new cgroups. This is also
how it knows where to query when doing things like tweaking cgroup settings and querying the
status of current cgroups.

<strong>USERNAME</strong> the reason why username is provided is because by default when a 
new cgroup is created all of the files within its directory are owned by root. This means
that only root can write to these files. And, with cgroups the way they are modified is by
writing to their files... To minimize the number of "root" commands run by the program (using
the <a href="https://crates.io/crates/runas">runas</a> crate) whenever a new cgroup is created (requiring root access)
the program will create the cgroup by making a new directory with the cgroup name. Additionally 
it will chown this directory so that USERNAME is the owner. This let's subsequent read and writes
to the directory be done by the user so you don't need to constantly use sudo.




