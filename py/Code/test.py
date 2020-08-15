import ast
import pprint

def convert_mode1(op1,op2,check=True):

    import numpy
    res = list(numpy.array(op1) + op2)

def convert_mode2(op1,op2,check=True):
    import operator
    res = list(map(operator.sub, op1, op2))

def convert_mode3(op1,op2,check=True):
    res = list(map(lambda x: x - op2, op1))
'''
def token_generation(content):
    line_of_codes = asttokens.ASTTokens(content, parse=True)
    for node in ast.walk(line_of_codes.tree):
        if hasattr(node, 'lineno'):
            print (node.__class__.__name__, line_of_codes.get_text(node))

'''
def visit_node():
    pass

def get_line_dict(parsed_content):
    tree = ast.parse(parsed_content)
    for node in tree.body:
        print(node,node.lineno)
        while isinstance(node, (ast.For, ast.While, ast.If)):
            print(lastBody)
            code_dict[node.lineno] = node.__class__.__name__

def get_tuple_data(node_object):
    pass

def parse_code(content):
    parsed_code = {"iterable":False,"op":[],"operation":"","optimizable":False}
    parsed_content = ast.parse(content)
    for node in ast.iter_child_nodes(parsed_content):
        keys = list(node.__dict__.keys())
        print(node.__dict__)
        if 'targets' in keys:
            if node.__dict__['targets'][0].__class__.__name__  == 'Tuple':
                get_tuple_data(node.__dict__['targets'][0])
            else:
                op_dict = dict()
                op_dict['id'] = node.__dict__['targets'][i]
                op_dict['type'] = node.__dict__['value'].__class__.__name__
                parsed_code['op'].append(op_dict)

        if 'body' in keys:
            for i in node.__dict__['body']:
                if i.__class__.__name__ == 'For':
                    visit_node()
                print(i.__class__.__name__)
        '''for attr in ast.iter_child_nodes(node):

            print(attr.__dict__,end="\n\n")
'''
    pprint.pprint(ast.dump(parsed_content))

code_dict = {}
with open('C:\\Users\\UBAID USMANI\\Desktop\\Projects\\Code_Reviewer\\Training_Data\\reverse.txt','r') as file:
    content = file.read()
    parse_code(content)