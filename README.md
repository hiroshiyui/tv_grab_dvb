# EPGrab

Grab DVB EPG data, output to XMLTV format.

EPGrab is a fork of 'tv\_grab\_dvb' from http://bryars.eu/projects/tv_grab_dvb/

## Minimum Requirements

* A Linux supported DVB receiver device, see: http://www.linuxtv.org/wiki/index.php/Hardware_Device_Information
* Have DVB apps (http://www.linuxtv.org/wiki/index.php/LinuxTV_dvb-apps) installed on your system.

## Build

I do use CMake build system:

* <code>cmake .</code>
* <code>make</code>

## Run

You have to use DVB apps' \*zap utilities to set up your DVB receiver. Before that, use 'scan' utility to generate a channels list file.

For instance:

<code>tzap -F -c ~/channels.conf "公共電視 PTS"</code>

Then you can run EPGrab's executable file 'epgrab' to grab DVB EPG data.

## License

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
