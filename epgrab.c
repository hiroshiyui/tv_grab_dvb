/*
    EPGrab, an utility which decodes DVB EPG data
    Copyright (C) 2020  YOU Hui-Hong <hiroshi@ghostsinthelab.org>

    This file is part of EPGrab.

    EPGrab is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    EPGrab is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with EPGrab.  If not, see <http://www.gnu.org/licenses/>.
 */

#include <stdio.h>

#include "config.h"

#include <linux/dvb/dmx.h>
#include <libdvbv5/dvb-dev.h>

int main( int argc, char *argv[] ) {
    // Find DVB devices
	struct dvb_device *dvb;
	struct dvb_dev_list *dvb_dev;
    dvb = dvb_dev_alloc();

    if ( dvb_dev_find(dvb, NULL, NULL) == 0 ) {
        printf("Devices: %d\n", dvb->num_devices);
    }

    // Find first frontend device
	dvb_dev = dvb_dev_seek_by_adapter(dvb, 0, 0, DVB_DEVICE_FRONTEND);
    if (dvb_dev != NULL) {
        printf("Device: %s (%s)\n", dvb_dev->sysname, dvb_dev->path);
    }

    // Find first demus device
	dvb_dev = dvb_dev_seek_by_adapter(dvb, 0, 0, DVB_DEVICE_DEMUX);
    if (dvb_dev != NULL) {
        printf("Device: %s (%s)\n", dvb_dev->sysname, dvb_dev->path);
    }

    dvb_dev_free(dvb);
    return 0;
}