---
external: false
title: "How does the blockchain work? Part 1"
description: "How does the blockchain work? Part 1"
date: 2020-07-31
---

Blockchain is a combination of several different technologies. For making it easier for you, I have broken it down into simpler parts. This is the first part of this series and it’s all about one of the most important technologies used in blockchain; ***cryptography***

What is cryptography?
The prefix “crypt” means “hidden” and the suffix “graphy” means “writing.” Since that is out of the way, let’s see what it is. But before that let’s do something fun. Let’s create a secret code. Exciting, isn’t it? Let’s say that we have to subtract ‘1’ from each of the letters in our code to find the original letter. Cool. Since we have created a code and both of us know how to read it, if I write “BQQMF” both of us can easily understand what that means. But if you show it to your friend who has no clue about our secret code, it’d just be a senseless word for him.

This is what cryptography is. It’s a technique to secure information and communications through codes which only the sender and receiver can understand. When I changed “APPLE” to “BQQMF” what I did was encryption. And when you cracked the code, what you did is called decryption

When you learn about a particular technique you’ll have to remember some terms associated with it. Like encryption and decryption, there are a bunch of other simple words associated with cryptography. Let’s see what they mean.

Let’s take our example. In the world of cryptography, the word APPLE which we encrypted is called plain text, and the code BQQMF is called ciphertext. You won’t be able to read BQQMF as APPLE if I didn’t tell you how to read it, that is, to subtract 1 from each letter. So ‘-1’ is what we call the key of our cryptographic algorithm. Without which we won’t be able to understand what each of us is saying.

Now let’s see some of the types of cryptography.

### Symmetric Key Cryptography
In this system both the sender and receiver use a single key for encryption and decryption. Our cryptographic algorithm is an example of Symmetric key cryptography.

### Asymmetric Key Cryptography
You can easily guess what this would mean from the previous definitions. Yes, you’re correct, in this cryptographic system, there are two types of keys, a ***public key*** and a ***private key***. The public key is used for encryption and the private key is used for decryption. Both the keys would be different. Even if the public key is known to everyone, only the intended receiver can decode it because he alone knows the private key.

Asymmetric encryption can be likened to a mailbox on the street. The mailbox is completely public — anyone who knows its location could go to it and drop in a letter. However, only the owner of the mailbox has a key which allows him to access it and read the letters. The location of the mailbox is the public key, something that is known to the public. The private key is with the owner of the mailbox, which is used to access the mailbox.

### Hash Functions
In this algorithm, there is no key. These functions change the entire plain text to a new encrypted form, called the hash value. The hash value will be of the same length for whatever plain text fed into it. Since there is no key it is nearly impossible for the contents of the plain text to be recovered.

Public key cryptography and hash functions are used throughout blockchain. There is also another interesting cryptographic technique used inside blockchain called ***Merkle Trees***. A Merkle tree (or hash tree) is a tree that utilizes cryptographic hash functions to store hash outputs instead of raw data in each node. We’ll see what they mean in-depth in the coming articles.