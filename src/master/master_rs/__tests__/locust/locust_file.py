#!/usr/bin/env python3

import locust



class Ping(locust.HttpUser):

    @locust.task
    def do_inp(self):
        self.client.get(url='')
