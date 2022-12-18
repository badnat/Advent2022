f = open("prime",'r')
filedata = f.read()
f.close()

newdata = filedata.replace(r"\r\n","\n")

f = open("primes",'w')
f.write(newdata)
f.close()