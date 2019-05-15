import ast,asttokens
import pprint

def convert_mode1(op1,op2,check=True,type):

    import numpy
    res = list(numpy.array(op1) + op2)

def convert_mode2(op1,op2,check=True,type):
    import operator
    res = list(map(operator.sub, op1, op2))

def convert_mode3(op1,op2,check=True,type):
    res = list(map(lambda x: x - op2, op1))

def token_generation(content):
    line_of_codes = asttokens.ASTTokens(content, parse=True)
    for node in ast.walk(line_of_codes.tree):
        if hasattr(node, 'lineno'):
            print (node.__class__.__name__, line_of_codes.get_text(node))


def get_line_dict(parsed_content):
    tree = ast.parse(parsed_content)
    for node in tree.body:
        print(node,node.lineno)
        while isinstance(node, (ast.For, ast.While, ast.If)):
            
        print(lastBody)
        code_dict[node.lineno] = node.__class__.__name__




def parse_code(content):
    parsed_code = {"iterable":False,"op1_type":"","op2_type":"","operation":"","optimizable":False}
    parsed_content = ast.parse(content)
    for node in ast.iter_child_nodes(parsed_content):
        for attr in ast.iter_child_nodes(node):

            print(ast.dump(attr),ast.dump(node),sep ="\t\t",end="\n\n")
            for i in ast.iter_fields(attr):
                print(i[0],i[1])
    pprint.pprint(ast.dump(parsed_content))



code_dict = {}
with open('reverse.txt','r') as file:
    content = file.read()
    get_line_dict(content)