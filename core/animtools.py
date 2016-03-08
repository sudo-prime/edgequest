# Animtools.py

# Animation tools

'''

This file has animation tools for drawing things to the screen in EdgeQuest.

* lightning_direction - changes the direction of
lightning/bullets/magic spells in the middle of its 'trajectory'

'''

# Functions --------------------------------------------------------------------

def lightning_direction(source_x, source_y, target_x, target_y):
    ''' Get a lightning bolt based on two coordinate pairs '''
    # The lightning bolt changes depending on where the target_coords are
    # Diagonal up-left down-right
    if (source_x < target_x and source_y < target_y) or \
    (source_x > target_x and source_y > target_y):
        return '\\'
    # Diagonal up-right, down-left
    elif (source_x < target_x and source_y > target_y) or \
    (source_x > target_x and source_y < target_y):
        return '/'
    # Up or down
    elif (source_x == target_x and source_y != target_y):
        return '|'
    # Left or right
    elif (source_x != target_x and source_y == target_y):
        return '-'
    else:
        return '?'

# ------------------------------------------------------------------------------