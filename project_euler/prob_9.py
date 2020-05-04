from tqdm import tqdm
from multiprocessing import Pool

if __name__ == '__main__':
    for i in tqdm(range(1, 1000)):
        for j in range(i, 1000):
            for x in range(j, 1000):
                if i**2 + j**2 == x**2 and i+j+x == 1000:
                    print(i, j, x, i*j*x)