"""description"""

import itertools
import pygame
from constants import *
from state import State


def draw_rect(surface, colour, rect):
        pygame.draw.rect(surface, colour, rect)


class Wall:
        def __init__(self, *args):
                if len(args) == 1:
                        self.x = args[0][0]
                        self.y = args[0][1]
                elif len(args) == 2:
                        self.x = args[0]
                        self.y = args[1]
                else:
                        raise TypeError(
                                "invalid number of arguments"
                        )

        def draw(self, surface):
                screen_width, screen_height = SCREEN_DIMENTIONS
                width = screen_width / WIDTH
                height = screen_height / HEIGHT
                rect = pygame.Rect(
                        width * self.x - SMALL,
                        height * self.y - SMALL,
                        width + SMALL,
                        height + SMALL,
                )
                draw_rect(surface, DARK_GREY, rect)

        def to_raw(self):
                return (self.x, self.y)

        def __eq__(self, other):

                return (self.x == other.x) and (self.y == other.y)


class RenderState(State):
        def __init__(self, program):
                # todo
                self.tool = 0
                self.settings = program.settings
                program.walls = [
                        Wall(wall) for wall in program.walls
                ]

        def tick(self, program):
                self.draw(program)
                if self.tool == 0:
                        if pygame.mouse.get_pressed()[0]:
                                self.tick_draw_tool(program)
                        elif pygame.mouse.get_pressed()[2]:
                                self.tick_erase_tool(program)
                if pygame.mouse.get_pressed()[0]:
                        if self.tool == 1:
                                self.tick_draw_tool(program)
                        elif self.tool == 2:
                                self.tick_e
                                rase_tool(program)

        def get_affected_by_tool(self, program):
                x,y = self.get_mouse_grid_ref()
                radius = program.settings.get_by_name("radius")
                x_list = range(
                        max(x - radius//2, 0), min(x + radius//2, WIDTH)
                )
                y_list = range(
                        max(y - radius//2, 0), min(y + radius//2, HEIGHT)
                )
                return itertools.product(x_list,y_list)


        def get_mouse_grid_ref(self):
                x, y = pygame.mouse.get_pos()
                box_x = round(x / SCREEN_DIMENTIONS[0] * WIDTH)
                box_y = round(y / SCREEN_DIMENTIONS[1] * HEIGHT)
                return box_x,box_y

        def tick_draw_tool(self, program):
                for x,y in self.get_affected_by_tool(program):

                        #box_x, box_y = self.get_mouse_grid_ref()
                        wall = Wall(x, y)
                        if wall not in program.walls:
                                program.walls.append(wall)

        def tick_erase_tool(self, program):
                for x,y in self.get_affected_by_tool(program):

                        #box_x, box_y = self.get_mouse_grid_ref()
                        wall = Wall(x, y)
                        if wall in program.walls:
                                program.walls.remove(wall)

        def draw(self, program):
                # background
                program.screen.fill((255, 255, 255))

                for wall in program.walls:
                        wall.draw(program.screen)

                for ant in program.ants:
                        ant.draw(program.screen)

                pygame.display.flip()
