""" Rushed class to simulate the gun """

import time
import random
import math

class Gun(object):

    STRING = "{t}: {x}, {y}, {z}"

    def __init__(self, x=0, y=0, z=0, r=1):
        self.x = x
        self.y = y
        self.z = z
        self.range = r

        random.seed()

        # Help the rust code find the origin
        self.aim(time=20)

    def aim(self, time=20):
        # define vars:
        direction_x = random.randrange(-1, 2)
        direction_y = random.randrange(-1, 2)
        direction_z = random.randrange(-1, 2)
        speed_x = (random.random() * self.range * .1) * direction_x
        speed_y = (random.random() * self.range * .1) * direction_y
        speed_z = (random.random() * self.range * .01) * direction_z

        for i in range(0, time):
            if random.randrange(0, 3) == 0:
                direction_x = random.randrange(-1, 2)
                speed_x = (random.random() * self.range * .1) * direction_x

            if random.randrange(0, 3) == 0:
                direction_y = random.randrange(-1, 2)
                speed_y = (random.random() * self.range * .1) * direction_y

            if random.randrange(0, 3) == 0:
                direction_z = random.randrange(-1, 2)
                speed_z = (random.random() * self.range * .01) * direction_z

            self.x += speed_x
            if self.x >= self.range:
                direction_x = -1
            elif self.x <= -self.range:
                direction_x = 1


            self.y += speed_y
            if self.y >= self.range:
                direction_y = -1
            elif self.y <= -self.range:
                direction_y = 1

            self.z += speed_z
            if self.z >= self.range:
                direction_z = -1
            elif self.z <= -self.range:
                direction_z = 1

            self.print_location()

    def shot(self, time=6):
        original = float(self.z)

        for i in range(0, math.floor(time/2)):
            self.x += random.random() * 8
            self.print_location()

        # return it to about its original
        while self.x >= original:
            self.x -= random.random() * 8
            self.print_location()

        self.x = original
        self.print_location()

    def reload(self, time=20):
        or_x = float(self.x)
        or_y = float(self.y)
        or_z = float(self.z)

        # Direction that cant be 0
        direction = random.randrange(-1, 2, 2)

        for i in range(0, math.floor(time)):
            self.x += random.random() * (10 * direction)
            self.y += random.random() * (10 * direction)
            self.z += random.random() * (10 * direction)

            self.print_location()

        steps = random.randrange(8, 15)

        # Distances
        x_dist = (self.x - or_x) / steps
        y_dist = (self.y - or_y) / steps
        z_dist = (self.z - or_z) / steps

        if direction == 1:
            while self.x >= or_x + 5:
                self.x -= x_dist
                self.y -= y_dist
                self.z -= z_dist
                self.print_location()
        else:
            while self.x < or_x - 5:
                self.x -= x_dist
                self.y -= y_dist
                self.z -= z_dist
                self.print_location()


    def sit(self, time=20):
        self.random(time=math.floor(time/2)) # be random until its sitting
        for i in range(0, math.ceil(time/2)):
            self.print_location()

    def random(self, time=20):
        # define vars:
        direction_x = random.randrange(-1, 2)
        direction_y = random.randrange(-1, 2)
        direction_z = random.randrange(-1, 2)
        speed_x = (random.random() * self.range) * direction_x
        speed_y = (random.random() * self.range) * direction_y
        speed_z = (random.random() * self.range) * direction_z

        for i in range(0, time):
            if random.randrange(0, 3) == 0:
                direction_x = random.randrange(-1, 2)
                speed_x = (random.random() * .1) * direction_x

            if random.randrange(0, 3) == 0:
                direction_y = random.randrange(-1, 2)
                speed_y = (random.random() * .1) * direction_y

            if random.randrange(0, 3) == 0:
                direction_z = random.randrange(-1, 2)
                speed_z = (random.random() * .1) * direction_z

            self.x += speed_x
            if self.x >= self.range:
                direction_x = -1
            elif self.x <= -self.range:
                direction_x = 1

            self.y += speed_y
            if self.y >= self.range:
                direction_y = -1
            elif self.y <= -self.range:
                direction_y = 1

            self.z += speed_z
            if self.z >= self.range:
                direction_z = -1
            elif self.z <= -self.range:
                direction_z = 1

            self.print_location()

    def print_location(self):
        print(self.STRING.format(
            t = ("%.6f" % time.time()).replace(".", ""),
            x = "%.5f" % self.x,
            y = "%.5f" % self.y,
            z = "%.5f" % self.z
        ))

    def stop(self):
        print("q")


if __name__ == "__main__":
    try:
        gun = Gun()

        # Edit code here
        gun.sit()
        gun.reload()
        gun.aim()
        gun.shot(10)

    except Exception as e:
        gun.stop()
        print(e)
    finally:
        # DO NOT EDIT
        gun.stop()
