

try:
    raise BaseException()
except BaseException as ex:
    print(ex)
    print(type(ex))
    # print(ex.__traceback__)
    # print(type(ex.__traceback__))