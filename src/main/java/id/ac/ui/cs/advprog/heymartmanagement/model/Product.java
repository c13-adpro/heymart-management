package id.ac.ui.cs.advprog.heymartmanagement.model;

import lombok.Builder;
import lombok.Getter;

@Builder
@Getter
public class Product {
    String productId;
    String productName;
    long productPrice;
    int productQuantity;

    public Product(String productName, long productPrice, int productQuantity) {}
}
