# CW20 I4I Escrow

This is an escrow meta-contract that allows multiple users to create independent
escrows. Each escrow involves two users and has a unique id (for future calls to
reference it) as well as other parameters that guard the state transitions that
are involved in the lifecycle of the escrow.

## Why

Creating a trustless design is nearly impossible when there is human involvement
in meeting or carrying out a commitment/obligation. To combat this problem, we 
created an escrow smart contract to hold individuals accountable to commitments 
and to compensate parties affected by dereliction of duties. From a high level, 
the escrow mechanism is designed to discourage users from behaving in a way that
negatively affects other users. The protocol is generic in design so that 
specification of terms can be programmed in. However, the overarching idea of 
the escrow smart contract is that actors who fall short of meeting obligations 
should be penalized, and the party affected should be compensated.

## How it works

Both parties, A and B, lock the same amount of tokens in the smart contract. 
Once the funds are locked, each party can send one of two messages to the smart 
contract: 

- Approve(secret) - A message containing the counterparty’s secret.
- Cancel - A message signifying the user’s desire to redeem a proportion of 
their deposit prior to completion, or because they failed to obtain the other 
user’s secret. 

Additionally, the escrow smart contract is configured with two timeouts: 

- T1: After the first deposit is paid into the escrow account, party B has T1 
time to send their deposit, after which party A will get their deposit back. T1 
is left to the discretion of the developer to determine. However, T1 is intended
to be relatively short, only a few minutes after the agreement is initially 
formed and the first party engages with the contract (i.e. sending tokens to the
 contract). 
- T2: T2 is a longer timeout that triggers a default action in case either of 
the parties fails to submit a message to the smart contract after both deposits 
have been paid. T2 is intended to cover the time it would take to fulfill the 
said obligation and is left to the discretion of the developer. Ex: if used for 
a delivery dapp and the delivery window is set to be 7 business days (BD). After
7 BD, if no message is submitted a default action is triggered.

The following table captures the sixteen possible states of the smart contract when
it comes time for disbursement. A tuple (x, y) signifies that user A receives x,
and user B receives y, where x and y are coefficients of their initial input. 
For example, (1.3, 0.7) indicates that user A receives 1.3 times their initial 
deposit, whilst user B only receives 0.7 times their initial deposit. In some 
cases, not all of it is returned to the users, and the remainder is sent to the 
protocol treasury. For example (0.5, 0.5) signifies that both users only 
retrieve 50% of their deposit, the rest goes to the treasury. This is meant to 
discourage parties from abandoning the formed agreement. Cancel(A) + Cancel(B), 
Cancel(X) + T2(X), or T2(A) + T2(B) combinations result in a portion or all 
tokens going to the treasury. Under no circumstances, if an Approve(secret) 
message is sent by a party can they lose a portion of their funds. They can only
receive back the initial portion locked or their initial portion plus a portion 
of other parties’ deposit.  


|                     | Approve(B,secret_a) | Cancel(B)  | T1(B)  | T2(B)      |
| :---                | :---:               | :---:      | :---:  | :---:      |  
|Approve(A, secret_b) | (1, 1)              | (1.3, 0.7) | X      | (1.5, 0.5) |            
|Cancel(A)            | (0.7, 1.3)          | (0.5, 0.5) | (1, 0) | (0.5, 0)   |
|T1(A)                | X                   | (0, 1)     | X      | (0, 1)     |
|T2(A)                | (0.5, 1.5)          | (0, 0.5)   | (1,0)  | (0, 0)     | 

**Approve(A, s_b) & Approve(B, s_a)**:

Both parties send each other their secrets before T2, everything is good, 
everyone gets their money back.

**Approve(A, s_b) & Cancel(B)**:

B actively canceled and sent their secret to A. A can submit the Approve message
using B’s secret. A receives 1.3, and B gets back 0.7.

**Approve(A, s_b) & T1(B)**:

Not possible because the smart-contract does not allow user A to Approve unless
B's account is funded.

**Approve(A, s_b) & T2(B)**:

B sent their secret to A, who was able to submit an Approve message with B’s 
secret but, for some reason, B was not able to cancel or send an Approve 
message before T2. In this case, B is penalized a bit more than if they had 
actively canceled because they made A wait for the expiry of the timeout.

**Cancel(A) & Approve(B,s_a)**:

Symmetrical to Approve(A, s_b) & Cancel(B)

**Cancel(A) & Cancel(B)**:

Both parties actively cancel, but neither is able to submit the other’s secret. 
They both get penalized the same amount, 0.5. Results in funds going to the 
treasury.

**Cancel(A) & T1(B)**:

User B never funded their account. After T1, user A can withdraw their full
deposit.

**Cancel(A) & T2(B)**:

A actively canceled, they never received B’s secret, and B timed out. This 
happens when one party is a no-show, they get penalized completely. Results in 
funds going to the treasury.

**T1(A) & Approve(A, s_b)**:

Not possible. Symmetrical to Approve(A, s_b) & T1(B).

**T1(A) & Cancel(B)**:

Symmetrical to Cancel(A) & T1(B)

**T1(A) & T1(B)**:

Not possible. Creating an escrow requires funding the creator's account. So at
least one account is funded.

**T1(A) & T2(B)**:

A never funded their account. User B can Cancel before T2 to get their funds 
back. But even if they wait until after T2, they will still get their deposit
back.

**T2(A) & Approve(B, s_a)**:

Symmetrical to Approve(A, s_b) & T2(B).

**T2(A) & Cancel(B)**:

Symmetrical to Cancel(A) & T2(B).

**T2(A) & T1(B)**:

Symmetrical to T1(A) & T2(B).

**T2(A) & T2(B)**:

Everyone loses their deposit. Results in funds going to the treasury.

## Exchange Secrets

TODO

## Notes

How are users compensated when their counterparties cancel or don’t fulfill an 
obligation? When the counterparty decides not to go ahead with a confirmed 
commitment, they can either actively cancel and send their secret code over to 
the other user or do nothing. If the counterparty sends their secret over, the 
user can submit the Approve(secret) message and get back 1.3 or 1.5 times their 
initial deposit, which is a suitable cancellation fee. The escrow mechanism 
incentivizes the counterparty to share their secret code as it enables them to 
at least retain a proportion of their initial deposit (0.5 or 0.7, depending on 
whether they successfully submit the cancel message before T2).

How does this fare in the face of bots programmed to automatically request or 
accept obligations with the objective of passively skimming rewards? The only 
way to get one’s full deposit back is to submit an OK message with the other 
party’s secret. However, a user will not communicate their secret code unless 
the other party satisfied their part of the commitment or if they themselves 
decided to cancel. More often than not, an automated bot will not obtain the 
counterparty’s secret code and is destined to lose money.

## Usage

Study the demo and scripts in the [nride-sc root directory](..)
