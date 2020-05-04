ones = ["", "one","two","three","four","five","six","seven","eight","nine"]
tens = ["","","twenty","thirty","forty","fifty","sixty","seventy","eighty","ninety"]
ten = ["ten","eleven","twelve","thirteen","fourteen","fifteen","sixteen","seventeen","eighteen","nighteen"]

def get_num(n):
    length = 0
    if n >= 1000:
        length += len("onethousand")
    if n >= 100:
        ones[(n%1000)//100]
sum(map(
    ,
    range(1000)
))