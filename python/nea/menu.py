from state import State
from constants import *
import pygame
# import random


class MenuItem:
        def __init__(self,text, position: pygame.Rect):
                self.text = text
                
                self.outer_rect = position
                self.text_rect = position.move(BORDER_HEIGHT,BORDER_HEIGHT)
                self.text_rect.clip(position)
                
                # self.text_rect.scale_by_ip(RECT_WIDTH,RECT_HEIGHT-BORDER_HEIGHT*2)
        
        def draw(self,surface):
                if self.outer_rect.collidepoint(pygame.mouse.get_pos()):
                        self.colour = DARK_GREY
                else:
                        self.colour = LIGHT_GREY
                pygame.draw.rect(surface,self.colour,self.outer_rect)
                font = pygame.font.Font(None, FONT_HEIGHT)
                text = font.render(self.text,True,(0,0,0))

                surface.blit(
                        text,
                        self.text_rect
                        )




        
        

class MenuState(State):
        def __init__(self,program):
                self.items = []
                self.register_menu_item("1")
                self.register_menu_item("2")
                self.register_menu_item("3")
                
                # todo: load settings
                pass

        def register_menu_item(self,text):
                self.items.append(MenuItem(
                        text,
                        pygame.Rect(
                                RECT_WIDTH/2,
                                RECT_HEIGHT*(2+1.25*len(self.items)),
                                RECT_WIDTH,
                                RECT_HEIGHT
                                )
                                ))

        def draw(self, program):
                # background
                program.screen.fill((255,255,255))

                for item,i in zip(self.items,range(len(self.items))):
                        item.draw(program.screen)

                # todo: draw settings

                pass

        def draw_item(item: MenuItem):
                pass

        def tick(self, program):
                self.draw(program)
                # process events for buttons
                pass

