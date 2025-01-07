#### Model

So we kind of want to make our own sdt for model creation and combination. When combining different models together we
need
to make sure, that the output of the sending model matches the input of the receiving model. To make that as easy as
possible
for the AIs user we introduce a json format what allows us to define our input and output. Also, this configuration
format
allows us to combine different models as long as lib-core covers the needed features. In Future here we also can
configure
a learning loop for live learning.
For now ´ai_config_example.json´ is our guide for the underlying functionality.

#### **Version**:

Declares the version of the config.

#### **Backend**:

Defines what the target for training is.

#### **Data**:

Here we define specifications for the **source** file. Like **type**, **path**, **has_header** and **delimiter**.
The specifications needed vary for different types.
Also, we can define the **scheme** of tables etc. as seen in the example, so we can specify the use of the data later
on.

#### **Transformation Pipeline**:

Here define how we want our data to be prepared, as seen in the example.

#### **Models**:

In models, we can see how we can assemble our networks. Every model got an **id** for mapping. The **model_type** gives
us
the model we want to use. **input_columns** is used to give the model a specific input. Every model needs different *
*hyperparams**
These handle things like **layers**, **activation** functions and more depending on the **model_type**.

#### **Output**:

Defines what **output** we are expecting at the end.