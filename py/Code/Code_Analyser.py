import numpy
import collections
import string
import re
class analyser:
    def __init__(self,doc):
        self.document = doc
        self.program_lines = {}
        self.parse_dict = {}
        self.space_factor = 1

    def lines_segregator(self):
        loop_count = 0
        with open(self.document,"r") as file:
            line = 0
            for i in list(file.readlines()):
                spaces = len(i) - len(i.lstrip())

                if spaces == 0 and 'for' not in i:
                    self.program_lines[i.strip()] = 'single'

                elif (('for' in i) and  (':' in i)  and (spaces == 0)):
                    loop_count += 1
                    self.program_lines[i.strip()] = str(loop_count)+ '_' + 'loop_start'

                elif 'for' in i and ':' in i and int(spaces/self.space_factor) >=1 :
                    loop_count += 1
                    self.program_lines[i.strip()] = str(loop_count) + '_' + 'loop_start'

                elif spaces>0:
                    loop_no = spaces / self.space_factor
                    self.program_lines[i.strip()] = str(loop_no) + "loop_lines"

                self.parse_tree(i.strip())
                print(self.program_lines)
                print(self.space_factor)

    def parse_tree(self,line):

        root = ""
        left_parent = ""
        right_parent = ""
        right_child = ""
        left_child = ""

        if '=' in line and 'print' not in line:
            root = '='
            parents = line.split(root)
            left_parent, right_parent = parents
            if '+' in list(right_parent):
                left_child, right_child = right_parent.split('+')
            elif '-' in list(right_parent):
                left_child, right_child = right_parent.split('-')
            elif '*' in list(right_parent):
                left_child, right_child = right_parent.split('*')
            elif '/' in list(right_parent):
                left_child, right_child = right_parent.split('/')

            self.parse_dict[line] = {'root': root, 'left_parent': left_parent, 'right_parent':right_parent, \
                                     'right_child':right_child, 'left_child':left_child}
        elif 'print' in line:
            root = line
            self.parse_dict[line] = {'root': root, 'left_parent': left_parent, 'right_parent':right_parent, \
                                     'right_child':right_child, 'left_child':left_child}



test_file = 'reverse.txt'
analyser(test_file).lines_segregator()



