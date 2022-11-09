KEY = {
    "d": 4756271488334244325464097983587403774265287914901729662095342340258501652149831561339489911670425523564313117249735566081017855610018282770556180726481535046101000233329314927462152965908230550234586552057851264436093741770402610571047685348840997263237184163888646310858862176979939574444446294008290198306045200899942073709206780976188591861311288818998615212697872320738496133037406823493225268268391743435345100540683044910600943410477455816982390783510071250093615753211429626634405020381532944457073528557424197583280653631937468019559685420341886707673077733339505099452249777979716469059984712191783780025121,
    "n": 17721704848318063139975028874301971307723728359369094345032010576841504009307667851885928914015603126811109873418432776308129649032605049737331803579746596049418766852451321035252399506530633460646534311125753434385332132945462985838302019299101918423078870585149356071789278133829982254343614085233406236152241812258684338013870303083792396781208969427584163059268948137401860804986806438606481039373066716823168859567500370521423409166362926842942126927587725033058027829779492427753208918382678387341034394994577939903266231698107840908510834047808070884351876921969030328452333552349267058109352284430788672905657,
}


def as_source(path, mode="rb"):
    if path and path != "-":
        return open(path, mode)
    from sys import stdin

    return stdin.buffer if "b" in mode else stdin


def as_sink(path, mode="wb"):
    if path and path != "-":
        return open(path, mode)
    from sys import stdout

    return stdout.buffer if "b" in mode else stdout


from binascii import hexlify
from struct import pack


def bytes2int(raw_bytes):
    # type: (bytes) -> int
    return int(hexlify(raw_bytes), 16)


def int2bytes(n):
    # type: (int) -> bytes
    if n < 0:
        raise ValueError("Negative numbers cannot be used: %i" % n)
    elif n == 0:
        return b"\x00"
    a = []
    while n > 0:
        a.append(pack("B", n & 0xFF))
        n >>= 8
    a.reverse()
    return b"".join(a)


def decrypt(crypto, n, d):
    return int2bytes(pow(bytes2int(crypto), d, n))


def decode_stream(src):
    # type: (IO) -> int
    """Read a varint from `src`"""
    b = src.read(1)
    if b:
        shift = result = 0
        while 1:
            i = ord(b)
            result |= (i & 0x7F) << shift
            if not (i & 0x80):
                break
            shift += 7
            b = src.read(1)
        return result
    else:
        return -1


def vdecrypt(n, d, src, out, i=0):
    from io import BytesIO

    s = decode_stream(src)
    while s > 0:
        cypher = src.read(s)
        blob = decrypt(cypher, n, d)
        # print('D', i, s, len(blob))
        b = BytesIO(blob)
        salt = decode_stream(b)
        index = decode_stream(b)
        block = b.read()
        assert index == i
        assert salt != 0
        out.write(block)
        i += 1
        s = decode_stream(src)


with as_source("") as r, as_sink("") as w:
    vdecrypt(KEY["n"], KEY["d"], r, w)
