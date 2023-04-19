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
<img src="https://drive.google.com/uc?export=view&id=1ThWIUlgFHa4_f5aF3o0cOSaa84dVaHPV">
<br>
<br>
These are used by the program as follows:
<br>
<strong>CGROUPROOT</strong> used to locate the root cgroup folder. This is the parent folder
(or root node in terms of cgroup hierarchy) where children cgroup folders will be created. The 
tool uses this location to create directories that will correspond to new cgroups. This is also
how it knows where to query when doing things like tweaking cgroup settings and querying the
status of current cgroups.
<br>
<strong>USERNAME</strong> the reason why username is provided is because by default when a 
new cgroup is created all of the files within its directory are owned by root. This means
that only root can write to these files. And, with cgroups the way they are modified is by
writing to their files... To minimize the number of "root" commands run by the program (using
the <a href="https://crates.io/crates/runas">runas</a> crate) whenever a new cgroup is created (requiring root access)
the program will create the cgroup by making a new directory with the cgroup name. Additionally 
it will chown this directory so that USERNAME is the owner. This let's subsequent read and writes
to the directory be done by the user so you don't need to constantly use sudo.
<br>
<strong>PATHJSON</strong> to help add support for existing cgroups, and for persitence for closing and opening
the app there is a file called "existing_cgroups.json" which will be discussed shortly. This file needs to be 
visible to the program for best results. And so this variable tells the program where to look.
<br>

<br>


<h4>existing_cgroups.json</h4>

This file as mentioned earlier helps support importing existing cgroups, and for keeping
track of when cgroups are added or deleted across program use. Essentially it makes sure the 
program is always up to date with your cgroups.

The file contents looks something like this:
<br>
<br>
<img src="https://drive.google.com/uc?export=view&id=1j3VPoOOuVPqQzJ9-hKDmRNz8usif54ga">
<br>

It's really simple json. You add cgroups that exist on the file system here by adding lines like the 
above. For cgroups that should be added use delete=0, for cgroups that should be deleted used delete=1.
<br>
<br>
<strong>(Important if you add cgroups manually, e.g. those that aren't created using this program <br>
be sure to chown the files in their directory to match the "USERNAME" global variable <br> otherwise you will
get permission denied)</strong>
<br>
<br>
When the program starts up it will add cgroups with delete=0 (after it confirms they exist on the computer) to it's cgroup vector which is seen across the whole program.
<br>
It will then look for those with delete=1, check that they exist, and if so ask for you to confirm their deletion. Using the example existing_cgroups.json file
shown earlier would yield the following?
<br>
<br>
<img src="https://drive.google.com/uc?export=view&id=1tvFF9s_M1xRZrR-svrRnm09k0UBjEb4x">
<br>
<br>
This will remove those cgroups (really just multiple rmdir commands) and update the existing_cgroups.json file immediately:

<br>
<br>
<img src="https://drive.google.com/uc?export=view&id=1yErCz4D4yuMmOQ9kOTZZe-ml5CyqFcRZ">
<br>
<br>

<h3>Using the Program</h3>

After following the above steps, the program is meant to be intuitive right out of the box. Upon startup it tells you the assumed
global variables and where to change them if you need. 
<br>
<br>
<img src="https://drive.google.com/uc?export=view&id=1VxIVDT1dlNzs77l_Sp7qXqArts8Z5-IT">
<br>
<br>
Additionally as seen above it tells you the active controllers which is just a read to the file {CGROUPROOT}/cgroups.subtree_control. It then
lets you modify which controls are active, Y/y for activate, N/n for deactivate, L/l, for leave as is.
<br><br>
Once the active controllers are set it confirms which ones are active and it lets you know the imported cgroups.
<br>
<br>
<img src="https://drive.google.com/uc?export=view&id=1Zm98agMRaCtStgIk0uKsb6QIidI8d-Dg">
<br>
<br>
It then checks if it's ok to delete any delete=1 cgroups as mentioed earlier.
<br><br>

Once the basic start up is finished you are greeted with a few different choices:
<br>
<br>
<img src="https://drive.google.com/uc?export=view&id=1SrSaYMU46VezOVH5oFLvo7RKJL5qi3xc">
<br>
<br>
Choosing "Manage controllers" essentially reruns you through the earlier active controllers choice in case you 
want to make any changes. "Create a Cgroup" is self explanatory, it will ask for a cgroup name and then create
it on the computer and also update existing_cgroups.json. The last choice "Manage a Cgroup" brings up the following 
options:

<br>
<br>
<img src="https://drive.google.com/uc?export=view&id=125Zd98VT2umHM_TE2IhJM-VGPwTyroqx">
<br>
<br>
The first two are essentially reading and setting the settings for a cgroup. So for example if I want to read the 
memory.max file for "mycgroup1" it would look like this:

<br>
<br>
<img src="https://drive.google.com/uc?export=view&id=1WmeIMXaw_7tGJ0peL7ZpF-0_tQAlxYCa">
<br>
<br>

<br>
<br>
<img src="https://drive.google.com/uc?export=view&id=1kY9SZkrYsbFSNvSlYAvsZK9m8IzfPGpw">
<br>
<br>


Then "Delete a Cgroup" is as the name says and "Add pid to cgroup" will let you add pid(s) to a specific cgroup.
These options are also as easy to use the above option so I won't go into detail here.

<br>
<br>

But anyways that is my project. If anyone uses this I would appreciate any feedback, and I hope it helps you in some way :) 


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
<img src="https://drive.google.com/uc?export=view&id=1ThWIUlgFHa4_f5aF3o0cOSaa84dVaHPV">
<br>
<br>
These are used by the program as follows:
<br>
<strong>CGROUPROOT</strong> used to locate the root cgroup folder. This is the parent folder
(or root node in terms of cgroup hierarchy) where children cgroup folders will be created. The 
tool uses this location to create directories that will correspond to new cgroups. This is also
how it knows where to query when doing things like tweaking cgroup settings and querying the
status of current cgroups.
<br>
<strong>USERNAME</strong> the reason why username is provided is because by default when a 
new cgroup is created all of the files within its directory are owned by root. This means
that only root can write to these files. And, with cgroups the way they are modified is by
writing to their files... To minimize the number of "root" commands run by the program (using
the <a href="https://crates.io/crates/runas">runas</a> crate) whenever a new cgroup is created (requiring root access)
the program will create the cgroup by making a new directory with the cgroup name. Additionally 
it will chown this directory so that USERNAME is the owner. This let's subsequent read and writes
to the directory be done by the user so you don't need to constantly use sudo.
<br>
<strong>PATHJSON</strong> to help add support for existing cgroups, and for persitence for closing and opening
the app there is a file called "existing_cgroups.json" which will be discussed shortly. This file needs to be 
visible to the program for best results. And so this variable tells the program where to look.
<br>

<br>


<h4>existing_cgroups.json</h4>

This file as mentioned earlier helps support importing existing cgroups, and for keeping
track of when cgroups are added or deleted across program use. Essentially it makes sure the 
program is always up to date with your cgroups.

The file contents looks something like this:
<br>
<br>
<img src="https://drive.google.com/uc?export=view&id=1j3VPoOOuVPqQzJ9-hKDmRNz8usif54ga">
<br>

It's really simple json. You add cgroups that exist on the file system here by adding lines like the 
above. For cgroups that should be added use delete=0, for cgroups that should be deleted used delete=1.
<br>
<br>
<strong>(Important if you add cgroups manually, e.g. those that aren't created using this program <br>
be sure to chown the files in their directory to match the "USERNAME" global variable <br> otherwise you will
get permission denied)</strong>
<br>
<br>
When the program starts up it will add cgroups with delete=0 (after it confirms they exist on the computer) to it's cgroup vector which is seen across the whole program.
<br>
It will then look for those with delete=1, check that they exist, and if so ask for you to confirm their deletion. Using the example existing_cgroups.json file
shown earlier would yield the following?
<br>
<br>
<img src="https://drive.google.com/uc?export=view&id=1tvFF9s_M1xRZrR-svrRnm09k0UBjEb4x">
<br>
<br>
This will remove those cgroups (really just multiple rmdir commands) and update the existing_cgroups.json file immediately:

<br>
<br>
<img src="https://drive.google.com/uc?export=view&id=1yErCz4D4yuMmOQ9kOTZZe-ml5CyqFcRZ">
<br>
<br>

<h3>Using the Program</h3>

After following the above steps, the program is meant to be intuitive right out of the box. Upon startup it tells you the assumed
global variables and where to change them if you need. 
<br>
<br>
<img src="https://drive.google.com/uc?export=view&id=1VxIVDT1dlNzs77l_Sp7qXqArts8Z5-IT">
<br>
<br>
Additionally as seen above it tells you the active controllers which is just a read to the file {CGROUPROOT}/cgroups.subtree_control. It then
lets you modify which controls are active, Y/y for activate, N/n for deactivate, L/l, for leave as is.
<br><br>
Once the active controllers are set it confirms which ones are active and it lets you know the imported cgroups.
<br>
<br>
<img src="https://drive.google.com/uc?export=view&id=1Zm98agMRaCtStgIk0uKsb6QIidI8d-Dg">
<br>
<br>
It then checks if it's ok to delete any delete=1 cgroups as mentioed earlier.
<br><br>

Once the basic start up is finished you are greeted with a few different choices:
<br>
<br>
<img src="https://drive.google.com/uc?export=view&id=1SrSaYMU46VezOVH5oFLvo7RKJL5qi3xc">
<br>
<br>
Choosing "Manage controllers" essentially reruns you through the earlier active controllers choice in case you 
want to make any changes. "Create a Cgroup" is self explanatory, it will ask for a cgroup name and then create
it on the computer and also update existing_cgroups.json. The last choice "Manage a Cgroup" brings up the following 
options:

<br>
<br>
<img src="https://drive.google.com/uc?export=view&id=125Zd98VT2umHM_TE2IhJM-VGPwTyroqx">
<br>
<br>
The first two are essentially reading and setting the settings for a cgroup. So for example if I want to read the 
memory.max file for "mycgroup1" it would look like this:

<br>
<br>
<img src="https://drive.google.com/uc?export=view&id=1WmeIMXaw_7tGJ0peL7ZpF-0_tQAlxYCa">
<br>
<br>

<br>
<br>
<img src="https://drive.google.com/uc?export=view&id=1kY9SZkrYsbFSNvSlYAvsZK9m8IzfPGpw">
<br>
<br>


Then "Delete a Cgroup" is as the name says and "Add pid to cgroup" will let you add pid(s) to a specific cgroup.
These options are also as easy to use the above option so I won't go into detail here.

<br>
<br>

But anyways that is my project. If anyone uses this I would appreciate any feedback, and I hope it helps you in some way :) 


