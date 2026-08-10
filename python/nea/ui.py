"""A file containing all state agnostic user interface elements,
like buttons and text boxes."""

import pygame
from constants import *


class Button:
        """A class that draws a button in the rect spesifyed,
        and renders text inside of the rect given."""

        def __init__(self, text, position: pygame.Rect):
                self.text = text
                self.hooks = {"mouse": []}

                self.outer_rect = position
                self.text_rect = position.move(
                        BORDER_HEIGHT, BORDER_HEIGHT
                )
                self.text_rect.clip(position)

        def draw(self, surface):
                if self.outer_rect.collidepoint(
                        pygame.mouse.get_pos()
                ):
                        self.colour = DARK_GREY
                else:
                        self.colour = LIGHT_GREY
                pygame.draw.rect(
                        surface, self.colour, self.outer_rect
                )
                font = pygame.font.Font(None, FONT_HEIGHT)
                text = font.render(self.text, True, (0, 0, 0))

                surface.blit(text, self.text_rect)

        def add_clicked_on_hook(self, hook, *args):
                self.hooks["mouse"].append([hook, args])

        def clicked_on(self):
                return bool(
                        pygame.mouse.get_pressed()[0]
                        and self.outer_rect.collidepoint(
                                pygame.mouse.get_pos()
                        )
                )

        def process_hooks(self):
                if self.clicked_on():
                        for hook, args in self.hooks["mouse"]:
                                hook(*(args[0]))


class TextBox:
        """A class that draws a button in the rect spesifyed,
        activating when clicked upon and recording data typed
        untill a click happens outside of the rect."""

        def __init__(self, position: pygame.Rect, program, text=""):
                self.text = text
                self.program = program
                self.active = False
                self.outer_rect = position
                self.width = position.width
                self.text_rect = position.move(
                        BORDER_HEIGHT, BORDER_HEIGHT
                )
                self.text_rect.clip(position)

        def draw(self, surface):
                if self.active or self.outer_rect.collidepoint(
                        pygame.mouse.get_pos()
                ):
                        self.colour = DARK_GREY
                else:
                        self.colour = LIGHT_GREY

                pygame.draw.rect(
                        surface, self.colour, self.outer_rect
                )

                font = pygame.font.Font(None, FONT_HEIGHT)
                text = font.render(self.text, True, (0, 0, 0))
                surface.blit(text, self.text_rect)
                self.outer_rect.w = max(
                        self.width, text.get_width() + BORDER_HEIGHT
                )

        def tick(self):
                if 0 != max(pygame.mouse.get_pressed()):
                        if self.outer_rect.collidepoint(
                                pygame.mouse.get_pos()
                        ):
                                self.active = True
                        else:
                                self.active = False
                for event in self.program.events:
                        if event.type == pygame.KEYDOWN:
                                if event.key == pygame.K_BACKSPACE:
                                        self.text = self.text[:-1]
                                        # python will not allow deleting empty space so validation not needed
                                else:
                                        self.text += event.unicode


# no tests in this file because doctests are for data in data out
# functions, not for validating the GUI
