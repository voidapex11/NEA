class State:
        """Parent class to all states the program can take."""

        def tick(self):
                raise NotImplementedError(
                        "Method not implemented by subclass"
                )

        def draw(self, program):
                raise NotImplementedError(
                        "Method not implemented by subclass"
                )
