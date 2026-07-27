from state import State
from constants import *
from ui import Button
import pygame, copy
# import random

class MenuItem(Button):
        def __init__(self, order_in_menu,*args, **kwargs):
                rect = pygame.Rect(
                                RECT_WIDTH/2,
                                RECT_HEIGHT*(2+1.25*order_in_menu),
                                RECT_WIDTH,
                                RECT_HEIGHT
                                )
                # based of off https://stackoverflow.com/questions/12701206/ddg#12701228
                super(MenuItem, self).__init__(*args,rect,**kwargs)


class CategoryButton(MenuItem):
        def __init__(self, settings, cat_index, order_in_menu,text, *args, **kwargs):
                # based of off https://stackoverflow.com/questions/12701206/ddg#12701228
                super(CategoryButton, self).__init__(order_in_menu,text,*args,**kwargs)
                if settings.get_all_from_category(text)!=[]:
                        settings.categorys.setdefault(
                                text,
                                settings.get_all_from_category(text)
                                )
                self.settings = settings
                self.index = cat_index
                self.max_outer_rect = self.outer_rect
                self.max_text_rect = self.text_rect
                self.update_spacing(cat_index)

                
                
        def update_spacing(self, cat_len):
                # todo: fix horisontal spacing
                if cat_len==0:
                        cat_len=1
                #cat_len +=1
                self.outer_rect = copy.deepcopy(self.max_outer_rect)
                self.text_rect = copy.deepcopy(self.max_text_rect)

                self.outer_rect.width /= cat_len
                #self.outer_rect.scale_by_ip(1/cat_len,1)
                
                
                #self.outer_rect.x += self.outer_rect.width*(self.index)
                delta_x = self.outer_rect.width*(self.index)
                self.outer_rect.move_ip(delta_x,0)
                self.text_rect.move_ip(delta_x,0)
                
                self.text_rect = self.text_rect.move(BORDER_HEIGHT,BORDER_HEIGHT)
                self.text_rect.clip(self.outer_rect)

                before = self.outer_rect.width
                self.outer_rect.width/=1.01
                after = self.outer_rect.width
                
                if not self.index==0:
                        self.outer_rect.x -= (after-before)#*self.index


class MenuState(State):
        def __init__(self,program):
                self.items = []
                self.categorys = []
                
                self.settings = program.settings
                self.register_menu_item("Start new simulation")
                # todo: only display continue button if there is a saved simulation
                
                self.register_menu_item("Continue prior simulation")
                
                for cat in self.settings.categorys:
                        self.register_category(cat)
                        self.categorys[-1].add_clicked_on_hook(
                                self.update_selected_category,
                                [len(self.categorys)]
                                )
                self.category = 0

                # todo: load settings
                pass

        def register_menu_item(self,text):
                self.items.append(MenuItem(len(self.items),text))
        
        def update_selected_category(self, new_category):
                self.category = new_category
        
        def register_category(self,text):
                self.categorys.append(CategoryButton(
                        self.settings, len(self.categorys),len(self.items),text
                ))
                for category in self.categorys:
                        category.update_spacing(len(self.categorys))

        def draw(self, program):
                # background
                program.screen.fill((255,255,255))

                for item,i in zip(self.items,range(len(self.items))):
                        item.draw(program.screen)

                for category,i in zip(self.categorys,range(len(self.categorys))):
                        category.update_spacing(len(self.categorys))
                        category.draw(program.screen)
                
                #import pdb;pdb.set_trace()

                # todo: draw settings

                pass

        def draw_item(item: MenuItem):
                pass

        def tick(self, program):
                self.draw(program)

                for item in self.items:
                        item.process_hooks()
                
                for category in self.categorys:
                        category.process_hooks()

                # process events for buttons
                pass

