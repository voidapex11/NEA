import pygame
import nea
from menu import MenuState



class Program:
    def __init__(self):
        pass

    def run(self):

        # Initialize Pygame
        pygame.init()

        self.screen = pygame.display.set_mode((1920/2, 1200/2))
        pygame.display.set_caption("Ant simulation")

        self.state = MenuState(self)


        # Game loop
        self.running = True
        while self.running:
                for event in pygame.event.get():
                        if event.type == pygame.QUIT:
                                self.running = False
                        else:
                                self.state.tick(self)
                pygame.display.update()


        # Quit Pygame
        pygame.quit()

app_instance = Program()
app_instance.run()