# recruiter-crypto
Stupid encoder/decoder command line tool for "funny" recruiters.

Sometimes on LinkedIn recruiters try to be mysterious by posting what looks like gibberish, for example (*1): 

```52 75 73 74 20 69 73 20 61 77 65 73 6f 6d 65 21 20 0a```

Most of the time they've just taken the text and printed out the numerical value (in hexadecimal or binary) of each character that makes up the text. This little command line tool encodes / decodes those types of messages. 

```
$ recruiter-crypto decode asciihex 
52 75 73 74
Rust
```

I would understand why if it was a real challenge and if it was for a security company or agency where the work might actually entail reverse engineering / breaking cryptos but for regular developer positions I just don't get it. The recruiters do it to stand out and I guess it must be working since they haven't stopped doing it. 

What would be more fun is if developers themselves posted challenges for the recruiters to pass :-) So if any recruiter stumbles upon this, here is a challenge for you: 

```ie vsp aimo m dzirlpis ims z oynz lvftxbdph au haqrsak pxjidpfvb gvpd uvhywfr md xx zhzu sag dlnfzmtj yu foxmg wwk raz ldm phhhxbf sjb nkvzvd obrp amzn ev bwdulqz```

*1 And no, that particular text they would never post for some reason.